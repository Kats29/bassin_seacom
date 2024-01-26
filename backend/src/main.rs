mod tcp_socket;
mod arm_backend;
mod driver_cn_pin;


fn main() -> sysfs_gpio::Result<()> {
    tcp_socket::tcp_listen();
    Ok(())
}

