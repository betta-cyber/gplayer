extern crate rodio;
extern crate reqwest;
extern crate tempfile;
#[macro_use]
extern crate serde;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod player;
mod fetch;
mod sink;

use std::thread;
use std::time::Duration;
use std::env;
use player::Player;
use sink::find;

fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    let backend = find(None).unwrap();

    let (player, _) = Player::new(move || (backend)(None));
    debug!("Playing...");
    player.load(&url, true);
    debug!("Done");
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
