use crate::{
    game::server::GameServer,
    net::protocol::{NetProtocol, parse_buffer},
};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn run_udp() {
    let socket = UdpSocket::bind("127.0.0.1:9000")
        .await
        .expect("Erro ao iniciar socket");
    println!("Servidor rodando...");
    let mut buf = [0u8; 1024];
    let mut game_server = GameServer::new();

    loop {
        let (len_bytes, addr) = socket
            .recv_from(&mut buf)
            .await
            .expect("Falha ao receber dados no buffer.");

        let msg = match parse_buffer(&buf[..len_bytes]) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[udp]: {:?}", e);
                continue;
            }
        };

        handle_protocol(msg, addr, &mut game_server, &socket).await;
    }
}

async fn handle_protocol(
    protocol: NetProtocol,
    addr: SocketAddr,
    gs: &mut GameServer,
    socket: &UdpSocket,
) {
    match protocol {
        NetProtocol::Connect => {
            let rooms = gs.get_rooms_view();
            let msg = format!(
                "CONNECTOK;{},{},{}|{},{},{}",
                rooms[0].id,
                rooms[0].player_count,
                rooms[0].max_players,
                rooms[1].id,
                rooms[1].player_count,
                rooms[1].max_players,
            );
            let buf = msg.as_bytes();
            let len = socket.send_to(buf, addr).await.unwrap();
            println!("{:?}", String::from_utf8_lossy(&buf[..len]));
        }
        NetProtocol::Join { room_id } => {
            let join_result = gs.join_player(addr, Some(room_id));
            let msg = format!(
                "JOINEDOK;player_id={} room_id={} players={} max={} is_left={}",
                join_result.player_id.unwrap_or(0),
                join_result.room_id.unwrap_or(0),
                join_result.players.unwrap_or(0),
                join_result.max.unwrap_or(0),
                join_result.is_left_player.unwrap_or(false)
            );
            if join_result.success {
                let buf = msg.as_bytes();
                let len = socket.send_to(buf, addr).await.unwrap();
                println!("{:?}", String::from_utf8_lossy(&buf[..len]));
            } else {
                let buf = b"JOIN_FAIL";
                let _ = socket.send_to(buf, addr);
            }
        }
        NetProtocol::Leave => {
            let room_id = gs.leave_player(addr);
            if let Some(room_id) = room_id {
                let msg = format!("LEFT room_id={:?}", room_id);
                let buf = msg.as_bytes();
                let len = socket.send_to(buf, addr).await.unwrap();
                println!("{:?}", String::from_utf8_lossy(&buf[..len]));
            };
        }
        NetProtocol::Ping => {
            //TODO
        }
    }
}
