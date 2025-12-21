use std::{collections::HashMap, net::SocketAddr};

pub struct GameServer {
    pub rooms: HashMap<u64, Room>,
    pub next_player_id: u64,
    pub next_room_id: u64,
}

impl GameServer {
    pub fn new() -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub id: u64,
    pub pos_y: f64,
    pub pos_x: f64,
    pub addr: SocketAddr,
    pub is_left_player: bool,
}

#[derive(Debug)]
pub struct Room {
    pub id: u64,
    pub players: HashMap<u64, Player>,
}
