pub enum NetProtocol {
    Join,
    Leave,
    Ping,
}

pub fn parse_buffer(bytes: &[u8]) -> Result<NetProtocol, ()> {
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
