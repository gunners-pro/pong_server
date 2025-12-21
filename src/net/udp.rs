use std::net::SocketAddr;
use tokio::net::UdpSocket;

use crate::game::server::{GameServer, Player};

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

        handle_connect(&mut game_server, addr).await;
    }
}

async fn handle_connect(server: &mut GameServer, addr: SocketAddr) {
    let player = Player {
        id: server.next_player_id,
        pos_y: 200.0,
        pos_x: 20.0,
        addr,
        is_left_player: true,
    };
    server.next_player_id += 1;
    for room in server.rooms.values_mut() {
        if room.players.len() < 2 {
            room.players.insert(player.id, player);
            break;
        }
    }
    for r in server.rooms.values() {
        println!("salas: {:?}", r);
    }
}
