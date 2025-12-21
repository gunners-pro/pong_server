use tokio::net::UdpSocket;

use crate::game::server::GameServer;

pub async fn run_udp() {
    let socket = UdpSocket::bind("127.0.0.1:9000")
        .await
        .expect("Erro ao iniciar socket");
    println!("Servidor rodando...");
    let mut buf = [0u8; 1024];
    let mut game_server = GameServer::new();

    loop {
        let (_, addr) = socket
            .recv_from(&mut buf)
            .await
            .expect("Falha ao receber dados no buffer.");

        let (joined, room_id, player_id, is_left) = game_server.join_player(addr);
        let msg = format!(
            "JOINED player_id={:?} room_id={:?} is_left={:?}",
            player_id, room_id, is_left
        );
        if joined {
            let buf = msg.as_bytes();
            let len = socket.send_to(buf, addr).await.unwrap();
            println!("{:?}", String::from_utf8_lossy(&buf[..len]));
        } else {
            let buf = b"JOIN_FAIL";
            let _ = socket.send_to(buf, addr);
        }
    }
}
