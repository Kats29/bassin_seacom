mod tcp_socket;

fn main() -> sysfs_gpio::Result<()> {
    tcp_socket::tcp_listen();
    Ok(())
}

