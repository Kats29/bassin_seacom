use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Duration;

use serde_json;
use websocket::{client::sync::Client, OwnedMessage, server::sync::Server, stream::sync::TcpStream};

use common::definitions::Command;

use crate::arm_backend::{ArmsBackend, ERR_LIST};

pub static STREAM: Mutex<RefCell<Option<Client<TcpStream>>>> = Mutex::new(RefCell::new(None));
pub static DRIVERS: Mutex<RefCell<Option<ArmsBackend>>> = Mutex::new(RefCell::new(None));

pub static MUTEX_UESED: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

fn handle_client() {

    let join_1 = std::thread::Builder::new().name("read_thread".to_string()).spawn(|| {
        loop {
            std::thread::sleep(Duration::new(1, 0));
            while match MUTEX_UESED.try_lock() {
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
                                            println!("Data received : \n{:?}", json);
                                            command = json;
                                            while match DRIVERS.try_lock() {
                                                Ok(_) => false,
                                                Err(_) => true
                                            } {};
                                            let update_result = vec![DRIVERS.lock().unwrap().borrow_mut().as_mut().unwrap().update(command)];
                                            println!(" list d'erreur :{:?}", ERR_LIST.lock().unwrap().borrow());
                                            let result = serde_json::to_string(&(0u32,if ERR_LIST.lock().unwrap().borrow().is_empty() {
                                                update_result
                                            } else {
                                                ERR_LIST.lock().unwrap().take()
                                            })).expect("Pb Json");
                                            println!("le petit json {:?}", result);
                                            borow_stream.as_mut().unwrap().send_message(&websocket::Message::text(result)).expect("TODO: panic message");
                                            *ERR_LIST.lock().unwrap().borrow_mut() = vec![];
                                        }
                                        Err(_) => {
                                            println!("Unrecognizable data : {}", msg);
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
    });
    let join_2 = std::thread::Builder::new().name("check_theard".to_string()).spawn(|| {
        loop {
            std::thread::sleep(Duration::new(5, 0));
            while match MUTEX_UESED.try_lock() {
                Ok(used) =>
                    {
                        if used.borrow().eq(&false) {
                            used.replace(true);
                            while match DRIVERS.try_lock() {
                                Ok(driv) => {
                                    println!("thread 2 pas finito pipo");
                                    let check = (1u32, driv.borrow().as_ref().unwrap().check_status());
                                    let result = serde_json::to_string(&check).expect("Pb Json");
                                    println!("Data a envoyer:  json {},vec : {:?}",result,check);
                                    while match STREAM.try_lock() {
                                        Ok(stream) => {
                                            match stream.borrow_mut().as_mut().unwrap().send_message(&websocket::Message::text(result.clone())) {
                                                Ok(_) => {
                                                    println!("Data envoyé dans le thread 2{}",result.clone());
                                                }
                                                Err(_) => {}
                                            }
                                            false
                                        },
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
                    std::thread::sleep(Duration::new(0, 500_000_000));
                    true
                }
            } {};
        }
    }).expect("TODO: panic message");
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
}

pub fn tcp_listen() -> std::io::Result<()> {
    let mut listener = Server::bind("0.0.0.0:3333")?;
    let arm = ArmsBackend::new().expect("Problème de démmarage (arm_back pas construit");
    DRIVERS.lock().unwrap().replace(Some(arm));

    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    while match listener.accept() {
        Ok(upgrade) => {
            let _ = STREAM.lock().unwrap().replace(Some(upgrade.accept().unwrap()));
            STREAM.lock().unwrap().borrow_mut().as_mut().unwrap().set_nonblocking(true).expect("Pb a non blocking");
            println!("New connection: {}", STREAM.lock().unwrap().borrow().as_ref().unwrap().peer_addr().unwrap());
            handle_client();
            true
        }
        Err(_) => {
            println!("Connection failed");
            false
        }
    } {}

    // close the socket server
    drop(listener);
    println!("TCP connection closed");
    Ok(())
}
