mod tcp_socket;
mod arm_backend;
mod driver_cn_pin;
mod drivers_cn_rs232;
mod error_handler;

use tcp_socket::tcp_listen;


fn main(){
    loop {
        match tcp_listen() {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
