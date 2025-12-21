mod game;
mod net;

use crate::net::udp::run_udp;

#[tokio::main]
async fn main() {
    run_udp().await;
}
