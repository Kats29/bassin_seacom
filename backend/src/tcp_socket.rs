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

use common::definitions::Command;

use crate::arm_backend::{
    ArmsBackend,
    ERR_LIST,
};
use crate::error_handler::{
    write_error_log,
    write_tcp_log,
};

pub static STREAM: Mutex<RefCell<Option<Client<TcpStream>>>> = Mutex::new(RefCell::new(None));
pub static DRIVERS: Mutex<RefCell<Option<ArmsBackend>>> = Mutex::new(RefCell::new(None));
pub static MUTEX_USED: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));
pub static STREAM_LOG_ERRORS: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));
pub static STREAM_LOG_IO: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));
pub static STREAM_LOG_TCP: Mutex<RefCell<Option<File>>> = Mutex::new(RefCell::new(None));

fn handle_client() -> std::io::Result<()> {
    let _join_1 = match Builder::new().name("update_thread".to_string()).spawn(|| {
        loop {
            sleep(Duration::new(1, 0));
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
                                                sleep(Duration::new(0, 500_000_000));
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
        }
    }) {
        Ok(jh) => jh,
        Err(e) => {
            write_error_log("Could not create the update thread".to_string());
            return Err(e);
        }
    };
    let _join_2 = match Builder::new().name("check_theard".to_string()).spawn(|| {
        loop {
            sleep(Duration::new(5, 0));
            while match MUTEX_USED.try_lock() {
                Ok(used) =>
                    {
                        if used.borrow().eq(&false) {
                            used.replace(true);
                            while match DRIVERS.try_lock() {
                                Ok(driv) => {
                                    let check = driv.borrow().as_ref().unwrap().check_status();
                                    let result = serde_json::to_string(&check).unwrap();
                                    while match STREAM.try_lock() {
                                        Ok(stream) => {
                                            while match stream.borrow_mut().as_mut().unwrap().send_message(&websocket::Message::text(result.clone())) {
                                                Ok(_) => {
                                                    write_tcp_log(format!("Data({:?}) send from Thread Check", result));
                                                    false
                                                }
                                                Err(_) => {
                                                    write_error_log(format!("Could not send data({:?}) from Thread Update", result));
                                                    true
                                                }
                                            } {
                                                sleep(Duration::new(0, 500_000_000));
                                            }
                                            false
                                        }
                                        Err(_) => true
                                    } {};

                                    false
                                }
                                Err(_) => true
                            } {};
                            used.replace(false);
                        }
                        false
                    }
                Err(_) => {
                    sleep(Duration::new(0, 500_000_000));
                    true
                }
            } {};
        }
    }) {
        Ok(jh) => jh,
        Err(e) => {
            write_error_log("Could not create the check thread".to_string());
            return Err(e);
        }
    };
    /*loop {
        match stream.recv_message() {
            Ok(OwnedMessage::Text(msg)) => {
                let command: Command;
                match serde_json::from_str(msg.as_str()) {
                    Ok(json) => {
                        println!("Data received : \n{:?}", json);
                        command = json;
                        let update_result = DRIVERS.lock().unwrap().borrow().update(command);
                        println!(" list d'erreur :{:?}", ERR_LIST.lock().unwrap());
                        let result = if ERR_LIST.lock().unwrap().is_empty() {
                            serde_json::to_string(&[update_result]).expect("Pb Json")
                        } else {
                            serde_json::to_string(ERR_LIST.lock().unwrap().deref()).expect("Pb Json")
                        };
                        stream.send_message(&websocket::Message::text(result)).expect("TODO: panic message");
                        *ERR_LIST.lock().unwrap() = vec![];
                    }
                    Err(ref e) => {
                        println!("Unrecognizable data : {}", msg);
                    }
                }
            }
            Ok(_) => {
                println!("un message pas au format text");
            }

            Err(ref e) => {
                // println!("encountered IO error: {e}");
            }
        }
        let check = DRIVERS.lock().unwrap().borrow().check_status();
        let result = serde_json::to_string(&[check]).expect("Pb Json");
        match stream.send_message(&websocket::Message::text(result)) {
            Ok(_) => {}
            Err(_) => {}
        }*/
    // loop {
    //     if join_1.is_finished() {
    //         println!("Le thread de lecture est finito pipo");
    //         break;
    //     }
    //     if join_2.is_finished() {
    //         println!("Le thread de checkup est finito pipo");
    //         break;
    //     }
    // }
    Ok(())
}

pub fn tcp_listen() -> std::io::Result<()> {
    STREAM_LOG_ERRORS.lock().unwrap().replace(
        Some(OpenOptions::new().append(true).create(true).open("./log/error.log").expect("Erreur ouverture fichier ./log/error.log"))
    );

    STREAM_LOG_TCP.lock().unwrap().replace(
        match OpenOptions::new().append(true).create(true).open("./log/tcp.log") {
            Ok(f) => Some(f),
            Err(e) => {
                write_error_log("Could not open ./log/tcp.log".to_string());
                return Err(e);
            }
        }
    );


    STREAM_LOG_IO.lock().unwrap().replace(
        match OpenOptions::new().append(true).create(true).open("./log/io.log") {
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
