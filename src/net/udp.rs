use std::net::SocketAddr;
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

enum NetProtocol {
    Join,
    Leave,
    Ping,
}

async fn handle_protocol(
    protocol: NetProtocol,
    addr: SocketAddr,
    gs: &mut GameServer,
    socket: &UdpSocket,
) {
    match protocol {
        NetProtocol::Join => {
            let (joined, room_id, player_id, is_left) = gs.join_player(addr);
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
        NetProtocol::Leave => {
            //TODO
        }
        NetProtocol::Ping => {
            //TODO
        }
    }
}

fn parse_buffer(bytes: &[u8]) -> Result<NetProtocol, ()> {
    if bytes.is_empty() {
        Err(()) //TODO
    } else {
        match str::from_utf8(bytes) {
            Ok(b) => {
                let b = b.trim_end();
                match b {
                    "Join" => Ok(NetProtocol::Join),
                    "Leave" => Ok(NetProtocol::Leave),
                    "Ping" => Ok(NetProtocol::Ping),
                    _ => Err(()), //TODO
                }
            }
            _ => Err(()), //TODO
        }
    }
}
