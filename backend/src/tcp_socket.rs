use std::{
    net::{TcpListener, TcpStream, Shutdown},
    io::Read,
    thread
};
use serde_json;
use common::definitions::Arm;

use crate::arm_backend::ArmsBackend;

fn handle_client(mut stream: TcpStream, mut drivers: ArmsBarckend) {
    let mut data = [0 as u8; 300]; // using 300 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            let (left, right): (Arm, Arm);
            match serde_json::from_slice(&data[0..size]) {
                Ok(json) => {
                    (left, right) = json;
                    drivers.update(left, right);
                    println!("Data received : \n{:?}\n{:?}", left, right);
                },
                Err(_) => println!("Unrecognizable data : {}", std::str::from_utf8(&data[0..size]).unwrap())
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn tcp_listen(mut drivers: ArmsBackend) -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream_res in listener.incoming() {
        let stream= stream_res?;
        println!("New connection: {}", stream.peer_addr().unwrap());
        thread::spawn(move || {
            handle_client(stream, drivers)
        });
    }
    // close the socket server
    drop(listener);
    println!("TCP connection closed");
    Ok(())
}
