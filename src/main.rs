use byteorder::{LittleEndian, WriteBytesExt};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::Mutex};

struct GameState {
    player1_pos: f32,
    player2_pos: f32,
    ball_x: f32,
    ball_y: f32,
    score1: u32,
    score2: u32,
}

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9000")
        .await
        .expect("Erro ao iniciar socket");
    println!("Servidor rodando em: 127.0.0.1:9000");

    let game_state = Arc::new(Mutex::new(GameState {
        player1_pos: 0.0,
        player2_pos: 0.0,
        ball_x: 400.0,
        ball_y: 0.0,
        score1: 0,
        score2: 0,
    }));

    let clients: Arc<Mutex<HashMap<String, SocketAddr>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut buf = [0u8; 1024];

    loop {
        let (len, addr) = socket
            .recv_from(&mut buf)
            .await
            .expect("Failed to receive data in the buffer.");

        if len < 2 {
            continue;
        } //pacote menor que 2 bytes é invalido, ignora.

        // registra client no hashmap
        {
            let mut cls = clients.lock().await;
            cls.insert(addr.to_string(), addr);
        }

        let player_id = buf[0];
        let action = buf[1];

        // Atualizar posição do jogador no game state
        {
            let mut gs = game_state.lock().await;
            let delta = match action {
                1 => -5.0, // up
                2 => 5.0,  // down
                _ => 0.0,
            };
            if player_id == 1 {
                gs.player1_pos += delta;
            } else if player_id == 2 {
                gs.player2_pos += delta;
            }
        }

        // Serializar estado para binario
        let gs = game_state.lock().await;
        let mut packet: Vec<u8> = Vec::new();
        packet.write_f32::<LittleEndian>(gs.player1_pos).unwrap();
        packet.write_f32::<LittleEndian>(gs.player2_pos).unwrap();
        packet.write_f32::<LittleEndian>(gs.ball_x).unwrap();
        packet.write_f32::<LittleEndian>(gs.ball_y).unwrap();
        packet.write_u32::<LittleEndian>(gs.score1).unwrap();
        packet.write_u32::<LittleEndian>(gs.score2).unwrap();

        // Enviar para todos os clientes
        let cls = clients.lock().await;
        for (_id, client_addr) in cls.iter() {
            let _ = socket.send_to(&packet, client_addr).await;
        }
    }
}
