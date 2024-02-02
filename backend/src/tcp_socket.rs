use websocket::{
    stream::sync::TcpStream,
    server::sync::Server,
    client::sync::Client,
    OwnedMessage
};
use serde_json;
use common::definitions::Command;
use common::error::HardwareError;
use crate::arm_backend::ArmsBackend;

fn handle_client(mut stream: Client<TcpStream>, drivers: &ArmsBackend) {
    while match stream.recv_message() {
        Ok(OwnedMessage::Text(msg)) => {
            let command: Command;
            match serde_json::from_str(msg.as_str()) {
                Ok(json) => {
                    command = json;
                    match drivers.clone().update(command){
                        Ok(_) => {}
                        Err(a) => {match a {
                            HardwareError::NotPowered => {}
                            HardwareError::NotStarted => {}
                            HardwareError::ArrMom => {}
                            HardwareError::ArrUrg => {}
                            HardwareError::OpenDoor(_) => {}
                            HardwareError::MovmentNotFinished(_) => {}
                            HardwareError::I2cCreation => {}
                            HardwareError::I2cSetSlave(_, _) => {}
                            HardwareError::I2cRead(_, _) => {}
                            HardwareError::I2cWrite(_, _) => {}
                            HardwareError::BadI2cResponse(_, _, _) => {}
                            HardwareError::PinExport(_) => {}
                            HardwareError::PinDirection(_) => {}
                            HardwareError::PinWrite(_) => {}
                            HardwareError::PinRead(_) => {}
                        }}
                    };
                    println!("Data received : \n{:?}", json);
                    true
                },
                Err(_) => {
                    println!("Unrecognizable data : {}", msg);
                    false
                }
            }
        },
        _ => {
            println!("An error occurred during connection with {}", stream.peer_addr().unwrap());
            false
        }
    } {}
}

pub fn tcp_listen(drivers: &mut ArmsBackend) -> std::io::Result<()> {
    let mut listener = Server::bind("0.0.0.0:3333")?;
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

     while match listener.accept() {
        Ok(upgrade) => {
            let stream = upgrade.accept().unwrap();
            println!("New connection: {}", stream.peer_addr().unwrap());
            handle_client(stream, &drivers);
            true
        },
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
