use std::{collections::HashMap, net::SocketAddr};
use tokio::net::UdpSocket;

#[derive(Debug, Clone, Copy)]
struct Player {
    id: u64,
    pos_y: f64,
    pos_x: f64,
    addr: SocketAddr,
    is_left_player: bool,
}

#[derive(Debug)]
struct Room {
    id: u64,
    players: HashMap<u64, Player>,
}

struct GameServer {
    rooms: HashMap<u64, Room>,
    next_player_id: u64,
    next_room_id: u64,
}

impl GameServer {
    fn new() -> Self {
        let mut rooms: HashMap<u64, Room> = HashMap::new();
        rooms.insert(
            1,
            Room {
                id: 1,
                players: HashMap::new(),
            },
        );
        rooms.insert(
            2,
            Room {
                id: 2,
                players: HashMap::new(),
            },
        );

        Self {
            rooms,
            next_player_id: 1,
            next_room_id: 3,
        }
    }
}

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
