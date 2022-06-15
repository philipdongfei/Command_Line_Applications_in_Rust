use log::{info, warn};
use env_logger;

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
