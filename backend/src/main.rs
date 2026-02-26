mod arm_backend;
mod driver_cn_pin;
mod drivers_cn_rs232;
mod error_handler;
mod tcp_socket;

use tcp_socket::tcp_listen;

fn main() {
    println!("Starting backend...");
    loop {
        match tcp_listen() {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
