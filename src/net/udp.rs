use crate::{
    game::server::{GameServer, JoinError},
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
            //Insere cada Cliente para futuro broadcast
            gs.clients.insert(addr);

            //chamar uma função para criar player ao se conectar com o cliente
            gs.create_player(addr);

            //envia valores iniciais da room ao se conectar com o cliente(game)
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
            let _ = socket.send_to(buf, addr).await.unwrap();
        }
        NetProtocol::Join { room_id } => match gs.join_player(addr, room_id) {
            Ok(result) => {
                gs.broadcast_rooms(&socket);

                let msg = format!(
                    "JOINEDOK;player_id={} room_id={} players={} max={}",
                    result.player_id.unwrap(),
                    result.room_id.unwrap(),
                    result.players.unwrap(),
                    result.max.unwrap(),
                );

                let _ = socket.send_to(msg.as_bytes(), addr).await;
            }

            Err(JoinError::AlreadyInRoom { room_id }) => {
                let msg = format!("JOIN_FAIL;reason=ALREADY_IN_ROOM room_id={}", room_id);
                let _ = socket.send_to(msg.as_bytes(), addr).await;
            }

            Err(JoinError::RoomFull) => {
                let msg = "JOIN_FAIL;reason=ROOM_FULL";
                let _ = socket.send_to(msg.as_bytes(), addr).await;
            }

            Err(JoinError::RoomNotFound) => {
                socket
                    .send_to(b"JOIN_FAIL;reason=ROOM_NOT_FOUND", addr)
                    .await
                    .unwrap();
            }

            Err(JoinError::PlayerNotFound) => {
                socket
                    .send_to(b"JOIN_FAIL;reason=PLAYER_NOT_FOUND", addr)
                    .await
                    .unwrap();
            }
        },
        NetProtocol::Leave { room_id } => {
            // let is_success = gs.leave_player(addr, room_id);
            // gs.broadcast_rooms(&socket);

            // if is_success {
            //     let msg = format!("LEAVEOK;room_id={}", room_id);
            //     let buf = msg.as_bytes();
            //     let len = socket.send_to(buf, addr).await.unwrap();
            //     println!("{:?}", String::from_utf8_lossy(&buf[..len]));
            // }
        }
        NetProtocol::Ping => {
            //TODO
        }
    }
}
