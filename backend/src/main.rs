use common::error::HardwareError;
use crate::tcp_socket::tcp_listen;

mod tcp_socket;
mod arm_backend;
mod driver_cn_pin;
mod drivers_cn_rs232;
mod error_handler;


fn main() -> Result<(),HardwareError> {
    let mut arm =  arm_backend::ArmsBackend::new()?;

    while match tcp_listen(&mut arm) {
        Ok(_) => false,
        Err(_) => true,
    }{}
    Ok(())

}
