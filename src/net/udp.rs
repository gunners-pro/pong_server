use tokio::net::UdpSocket;

pub async fn run_udp() {
    let socket = UdpSocket::bind("127.0.0.1:9000")
        .await
        .expect("Erro ao iniciar socket");
    println!("Servidor rodando...");
    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket
            .recv_from(&mut buf)
            .await
            .expect("Falha ao receber dados no buffer.");
    }
}
