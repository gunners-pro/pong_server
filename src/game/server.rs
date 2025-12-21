use std::{collections::HashMap, net::SocketAddr};

pub struct GameServer {
    pub rooms: HashMap<u64, Room>,
    pub next_player_id: u64,
    pub next_room_id: u64,
}

impl GameServer {
    pub fn new() -> Self {
        let mut rooms: HashMap<u64, Room> = HashMap::new();

        for i in 1..3 {
            rooms.insert(
                i,
                Room {
                    id: i,
                    players: HashMap::new(),
                },
            );
        }

        Self {
            rooms,
            next_player_id: 1,
            next_room_id: 3,
        }
    }

    pub fn join_player(
        &mut self,
        addr: SocketAddr,
    ) -> (bool, Option<u64>, Option<u64>, Option<bool>) {
        for (room_id, room) in self.rooms.iter_mut() {
            if room.players.values().any(|player| player.addr == addr) {
                continue;
            }

            if room.players.len() < 2 {
                let player = Player {
                    id: self.next_player_id,
                    pos_y: 200.0,
                    pos_x: 20.0,
                    addr,
                    is_left_player: if room.players.is_empty() { true } else { false },
                };
                self.next_player_id += 1;

                room.players.insert(player.id, player);
                return (
                    true,
                    Some(*room_id),
                    Some(player.id),
                    Some(player.is_left_player),
                );
            }
        }

        (false, None, None, None)
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
