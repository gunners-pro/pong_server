use crate::game::views::RoomInfo;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum JoinError {
    AlreadyInRoom { room_id: u64 },
    RoomNotFound,
    RoomFull,
}

pub struct GameServer {
    pub rooms: HashMap<u64, Room>,
    pub next_player_id: u64,
    pub next_room_id: u64,
    pub clients: HashSet<SocketAddr>,
}

pub struct JoinPlayerResult {
    pub success: bool,
    pub room_id: Option<u64>,
    pub player_id: Option<u64>,
    pub players: Option<u64>,
    pub max: Option<u64>,
    pub is_left_player: Option<bool>,
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
            clients: HashSet::new(),
            next_player_id: 1,
            next_room_id: 3,
        }
    }

    pub fn join_player(
        &mut self,
        addr: SocketAddr,
        room_to_join_id: Option<u64>,
    ) -> Result<JoinPlayerResult, JoinError> {
        if let Some(existing_room) = self.find_player_room(addr) {
            return Err(JoinError::AlreadyInRoom {
                room_id: existing_room,
            });
        }

        for (room_id, room) in self.rooms.iter_mut() {
            if let Some(target_id) = room_to_join_id {
                if *room_id != target_id {
                    continue;
                }
            }

            if room.players.len() >= 2 {
                return Err(JoinError::RoomFull);
            }

            let player = Player {
                id: self.next_player_id,
                pos_y: 200.0,
                pos_x: 20.0,
                addr,
                is_left_player: room.players.is_empty(),
            };
            self.next_player_id += 1;

            room.players.insert(player.id, player);

            return Ok(JoinPlayerResult {
                success: true,
                room_id: Some(*room_id),
                player_id: Some(player.id),
                players: Some(room.players.len() as u64),
                max: Some(2),
                is_left_player: Some(player.is_left_player),
            });
        }

        Err(JoinError::RoomNotFound)
    }

    pub fn leave_player(&mut self, addr: SocketAddr, room_id: u64) -> bool {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            let player_to_remove = room
                .players
                .iter()
                .find(|(_, player)| player.addr == addr)
                .map(|(key, _)| *key);

            if let Some(key) = player_to_remove {
                room.players.remove(&key);
                return true;
            }
        }
        false
    }

    pub fn get_rooms_view(&self) -> Vec<RoomInfo> {
        let mut room_info = Vec::new();
        for room in self.rooms.values() {
            room_info.push(RoomInfo {
                id: room.id,
                player_count: room.players.len() as u64,
                max_players: 2,
            });
        }
        room_info
    }

    pub fn broadcast_rooms(&self, socket: &UdpSocket) {
        let rooms = self.get_rooms_view();
        let payload = rooms
            .iter()
            .map(|r| format!("{},{},{}", r.id, r.player_count, r.max_players))
            .collect::<Vec<_>>()
            .join("|");

        let msg = format!("CONNECTOK;{}", payload);

        for addr in &self.clients {
            let _ = socket.try_send_to(msg.as_bytes(), *addr);
        }
    }

    fn find_player_room(&self, addr: SocketAddr) -> Option<u64> {
        for (room_id, room) in &self.rooms {
            if room.players.values().any(|p| p.addr == addr) {
                return Some(*room_id);
            }
        }
        None
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

#[derive(Debug, Clone)]
pub struct Room {
    pub id: u64,
    pub players: HashMap<u64, Player>,
}
