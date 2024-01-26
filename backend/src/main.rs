mod tcp_socket;
mod arm_backend;
mod driver_cn_pin;
mod drivers_cn_rs232;

fn main() -> std::io::Result<()> {
    tcp_socket::tcp_listen(arm_backend::ArmBackend::default())
}
