mod tcp_socket;
mod arm_backend;

fn main() -> std::io::Result<()> {
    tcp_socket::tcp_listen(arm_backend::ArmBackend::default())
}
