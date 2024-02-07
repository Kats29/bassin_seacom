use common::error::HardwareError;
use crate::tcp_socket::tcp_listen;

mod tcp_socket;
mod arm_backend;
mod driver_cn_pin;
mod drivers_cn_rs232;
mod error_handler;


fn main() -> Result<(),HardwareError> {
    while match tcp_listen() {
        Ok(_) => false,
        Err(_) => true,
    }{}
    Ok(())

}
