pub enum NetProtocol {
    Connect,
    Join { room_id: u64 },
    Leave { room_id: u64 },
    Ready { room_id: u64 },
    Ping,
}

pub fn parse_buffer(bytes: &[u8]) -> Result<NetProtocol, ()> {
    let s = std::str::from_utf8(bytes).map_err(|_| ())?.trim_end();
    let mut parts = s.splitn(2, ';'); // divide em até 2 partes
    let cmd = parts.next().ok_or(())?;
    let payload = parts.next();
    match cmd {
        "Connect" => Ok(NetProtocol::Connect),
        "Join" => {
            if let Some(payload) = payload {
                let room_id_str = payload.split('=').nth(1).ok_or(())?;
                let room_id = room_id_str.parse::<u64>().map_err(|_| ())?;
                Ok(NetProtocol::Join { room_id })
            } else {
                Err(()) // Join sem room_id
            }
        }
        "Leave" => {
            if let Some(payload) = payload {
                let room_id_str = payload.split('=').nth(1).ok_or(())?;
                let room_id = room_id_str.parse::<u64>().map_err(|_| ())?;
                Ok(NetProtocol::Leave { room_id })
            } else {
                Err(()) // Join sem room_id
            }
        }
        "Ready" => {
            if let Some(payload) = payload {
                let room_id_str = payload.split('=').nth(1).ok_or(())?;
                let room_id = room_id_str.parse::<u64>().map_err(|_| ())?;
                Ok(NetProtocol::Ready { room_id })
            } else {
                Err(()) // Join sem room_id
            }
        }
        "Ping" => Ok(NetProtocol::Ping),
        _ => Err(()),
    }
}
