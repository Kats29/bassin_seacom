use std::{
    cell::RefCell,
    sync::Mutex,
    thread::{
        Builder,
        sleep,
    },
    time::Duration,
};
use std::fs::{
    File,
    OpenOptions,
};
use std::io::ErrorKind;

use serde_json;
use websocket::{
    client::sync::Client,
    OwnedMessage,
    server::sync::Server,
    stream::sync::TcpStream,
};

use common::definitions::{Command, Status};

use crate::arm_backend::{
    ArmsBackend,
    ERR_LIST,
};
use crate::error_handler::{
    write_error_log,
    write_tcp_log,
};

/// Mutex du stream de la connexion TCP
pub static STREAM: Mutex<RefCell<Option<Client<TcpStream>>>> = Mutex::new(RefCell::new(None));
/// Mutex du [`ArmsBackend`] controlant le bassin
pub static DRIVERS: Mutex<RefCell<Option<ArmsBackend>>> = Mutex::new(RefCell::new(None));
/// Mutex de l'utilisation des autre mutex (évite les famines)
pub static MUTEX_USED: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
/// Mutex du [`File`] du fichier de log erreurs
pub static STREAM_LOG_ERRORS: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));
/// Mutex du [`File`] du fichier de log input/output hardware
pub static STREAM_LOG_IO: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));
/// Mutex du [`File`] du fichier de log tcp
pub static STREAM_LOG_TCP: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));
/// Constante du temps de veille du thread d'update
static SLEEP_TIME_UPDATE: Duration = Duration::new(0, 500_000_000);
/// Constante du temps de veille du thread de status
static SLEEP_TIME_STATUS: Duration = Duration::new(0, 500_000_000);

/// Boucle attandant la reception sur la connexion tcp d'une [`common::definitions::Command`] au format JSON.
/// La command est ensuite executé par [`ArmBackend::update`]
/// Un [`vec<common::error::HardwareError>`] au format JSON est ensuite envoyé en retour sur la connexion tcp
fn update_thread_function() {
    loop {
        while match MUTEX_USED.try_lock() {
            Ok(used) =>
                {
                    if used.borrow().eq(&false) {
                        used.replace(true);
                        let stream = STREAM.lock().unwrap();
                        let mut borow_stream = stream.borrow_mut();
                        match borow_stream.as_mut().unwrap().recv_message() {
                            Ok(OwnedMessage::Text(msg)) => {
                                let command: Command;
                                match serde_json::from_str(msg.as_str()) {
                                    Ok(json) => {
                                        write_tcp_log(format!("Data({:?}) received in  Thread Update", json));
                                        command = json;
                                        while match DRIVERS.try_lock() {
                                            Ok(_) => false,
                                            Err(_) => true
                                        } {};
                                        let update_result = vec![DRIVERS.lock().unwrap().borrow_mut().as_mut().unwrap().update(command)];
                                        let result = serde_json::to_string(&
                                            if ERR_LIST.lock().unwrap().borrow().is_empty() {
                                                update_result
                                            } else {
                                                ERR_LIST.lock().unwrap().take()
                                            }
                                        ).unwrap();
                                        while match borow_stream.as_mut().unwrap().send_message(&websocket::Message::text(result.clone())) {
                                            Ok(_) => {
                                                write_tcp_log(format!("Data({:?}) send from Thread Update", result));
                                                false
                                            }
                                            Err(_) => {
                                                write_error_log(format!("Could not send data({:?}) from Thread Update", result));
                                                true
                                            }
                                        } {
                                            sleep(SLEEP_TIME_UPDATE);
                                        }
                                        *ERR_LIST.lock().unwrap().borrow_mut() = vec![];
                                    }
                                    Err(_) => {
                                        write_error_log(format!("Unrecognizable data({}) received in Thread Update", msg));
                                    }
                                }
                            }
                            _ => {}
                        }
                        used.replace(false);
                    }
                    false
                }
            Err(_) => {
                true
            }
        } {};
        sleep(SLEEP_TIME_UPDATE);
    }
}


/// Boucle vérifiant le status du bassin.
/// Utilise la fonction [`ArmsBackend::check_status`] pour recevoir ce status. Si le status a changé depuis le dernier appel a la fonction,
/// le nouveau  [`common::definitions::Status`] est envoyé via la connexion TCP au format JSON
fn status_thread_function() {
    let mut status = Status::default();
    loop {
        while match MUTEX_USED.try_lock() {
            Ok(used) =>
                {
                    if used.borrow().eq(&false) {
                        used.replace(true);
                        while match DRIVERS.try_lock() {
                            Ok(driv) => {
                                let check = driv.borrow().as_ref().unwrap().check_status();
                                if check.ne(status) {
                                    status = check;
                                    let result = serde_json::to_string(&status).unwrap();
                                    while match STREAM.try_lock() {
                                        Ok(stream) => {
                                            while match stream.borrow_mut().as_mut().unwrap().send_message(&websocket::Message::text(result.clone())) {
                                                Ok(_) => {
                                                    write_tcp_log(format!("Data({:?}) send from Thread Check", result));
                                                    false
                                                }
                                                Err(_) => {
                                                    write_error_log(format!("Could not send data({:?}) from Thread Update", result));
                                                    false
                                                }
                                            } {
                                                sleep(SLEEP_TIME_STATUS);
                                            }
                                            false
                                        }
                                        Err(_) => true
                                    } {};
                                }
                                false
                            }
                            Err(_) => true
                        } {};
                        used.replace(false);
                    }
                    false
                }
            Err(_) => {
                sleep(SLEEP_TIME_STATUS);
                true
            }
        } {};


        sleep(Duration::new(0, 500_000));
    }
}

/// Fonction principale de notre serveur serveur WebSocket.
/// Créer deux thread, l'un avec la fonction  [`update_thread_function`] et l'autre avec la focntion
/// [`status_thread_function`], afin que ces deux fonctions tournent en parallèle.
fn handle_client() -> std::io::Result<()> {
    let _join_1 = match Builder::new().name("update_thread".to_string()).spawn(|| {
        update_thread_function()
    }) {
        Ok(jh) => jh,
        Err(e) => {
            write_error_log("Could not create the update thread".to_string());
            return Err(e);
        }
    };
    let _join_2 = match Builder::new().name("status_theard".to_string()).spawn(|| {
        status_thread_function()
    }) {
        Ok(jh) => jh,
        Err(e) => {
            write_error_log("Could not create the status thread".to_string());
            return Err(e);
        }
    };
    Ok(())
}
/// Ouvre les différents fichiers de log et stock  puis créer un serveur web socket qui écoute sur l'adresse localhost:3333.
/// Après une connexion la fonction [`handle_client`] est appelée.
pub fn tcp_listen() -> std::io::Result<()> {
    STREAM_LOG_ERRORS.lock().unwrap().replace(
        Some(OpenOptions::new()/*.write(true)*/.append(true).create(true).open("./log/error.log").expect("Erreur ouverture fichier ./log/error.log"))
    );

    STREAM_LOG_TCP.lock().unwrap().replace(
        match OpenOptions::new()/*.write(true)*/.append(true).create(true).open("./log/tcp.log") {
            Ok(f) => Some(f),
            Err(e) => {
                write_error_log("Could not open ./log/tcp.log".to_string());
                return Err(e);
            }
        }
    );


    STREAM_LOG_IO.lock().unwrap().replace(
        match OpenOptions::new()/*.write(true)*/.append(true).create(true).open("./log/io.log") {
            Ok(f) => Some(f),
            Err(e) => {
                write_error_log("Could not open ./log/io.log".to_string());
                return Err(e);
            }
        }
    );

    let mut listener = Server::bind("0.0.0.0:3333")?;

    let arm =
        match ArmsBackend::new() {
            Ok(ab) => ab,
            Err(he) => {
                write_error_log("Could not create ArmsBackend".to_string());
                return Err(std::io::Error::new(ErrorKind::Interrupted, he));
            }
        };
    DRIVERS.lock().unwrap().replace(Some(arm));

    // accept connections and process them, spawning a new thread for each one
    write_tcp_log("Server listening on port 3333".to_string());

    while match listener.accept() {
        Ok(upgrade) => {
            let _ = STREAM.lock().unwrap().replace(Some(upgrade.accept().unwrap()));

            match STREAM.lock().unwrap().borrow_mut().as_mut().unwrap().set_nonblocking(true) {
                Ok(_) => {}
                Err(e) => {
                    write_error_log("Could not set the tcp stream to non blocking".to_string());
                    return Err(e);
                }
            };
            write_tcp_log(format!("New connection: {}", STREAM.lock().unwrap().borrow().as_ref().unwrap().peer_addr().unwrap()));
            handle_client()?;
            true
        }
        Err(_) => {
            write_tcp_log("New connection failed".to_string());
            false
        }
    } {}

    // close the socket server
    drop(listener);
    write_tcp_log("TCP connection closed".to_string());
    Ok(())
}
