use crate::game::views::RoomInfo;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum JoinError {
    AlreadyInRoom { room_id: u64 },
    PlayerNotFound,
    RoomNotFound,
    RoomFull,
}

pub struct GameServer {
    pub rooms: HashMap<u64, Room>,
    pub players: HashMap<SocketAddr, Player>,
    pub next_player_id: u64,
    pub clients: HashSet<SocketAddr>,
}

pub struct JoinPlayerResult {
    pub room_id: Option<u64>,
    pub player_id: Option<u64>,
    pub players: Option<u64>,
    pub max: Option<u64>,
}

impl GameServer {
    pub fn new() -> Self {
        let mut rooms: HashMap<u64, Room> = HashMap::new();

        for i in 1..=2 {
            rooms.insert(
                i,
                Room {
                    players: HashSet::new(),
                    max_players: 2,
                },
            );
        }

        Self {
            rooms,
            players: HashMap::new(),
            clients: HashSet::new(),
            next_player_id: 1,
        }
    }

    pub fn create_player(&mut self, addr: SocketAddr) {
        let player = Player {
            id: self.next_player_id,
            addr,
            room_id: None,
            is_ready: false,
        };

        self.players.insert(addr, player);
        self.next_player_id += 1;
    }

    pub fn join_player(
        &mut self,
        addr: SocketAddr,
        room_to_join_id: u64,
    ) -> Result<JoinPlayerResult, JoinError> {
        //pega o player pra adicionar
        let player = self
            .players
            .get_mut(&addr)
            .ok_or(JoinError::PlayerNotFound)?;

        if let Some(current_room) = player.room_id {
            return Err(JoinError::AlreadyInRoom {
                room_id: current_room,
            });
        }

        let room = self
            .rooms
            .get_mut(&room_to_join_id)
            .ok_or(JoinError::RoomNotFound)?;

        if room.players.len() >= room.max_players {
            return Err(JoinError::RoomFull);
        }

        player.room_id = Some(room_to_join_id);
        room.players.insert(player.id);

        return Ok(JoinPlayerResult {
            room_id: player.room_id,
            player_id: Some(player.id),
            players: Some(room.players.len() as u64),
            max: Some(room.max_players as u64),
        });
    }

    // pub fn leave_player(&mut self, addr: SocketAddr, room_id: u64) -> bool {
    //     if let Some(room) = self.rooms.get_mut(&room_id) {
    //         let player_to_remove = room
    //             .players
    //             .iter()
    //             .find(|(_, player)| player.addr == addr)
    //             .map(|(key, _)| *key);

    //         if let Some(key) = player_to_remove {
    //             room.players.remove(&key);
    //             return true;
    //         }
    //     }
    //     false
    // }

    pub fn get_rooms_view(&self) -> Vec<RoomInfo> {
        let mut room_info = Vec::new();
        for (room_id, room) in &self.rooms {
            room_info.push(RoomInfo {
                id: *room_id,
                player_count: room.players.len() as u64,
                max_players: room.max_players as u64,
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
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u64,
    pub addr: SocketAddr,
    pub room_id: Option<u64>,
    pub is_ready: bool,
}

pub struct Room {
    pub players: HashSet<u64>,
    pub max_players: usize,
}
