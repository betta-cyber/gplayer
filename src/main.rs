extern crate rodio;
extern crate reqwest;
extern crate tempfile;
#[macro_use]
extern crate serde;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate minimp3;

mod player;
mod fetch;
mod sink;

use std::thread;
use std::time::Duration;
use std::env;
use player::Player;
use sink::find;
use std::fs::File;
use minimp3::{Decoder, Frame, Error};
use rodio::Sink;
use rodio::buffer::SamplesBuffer;

fn main() {
    pretty_env_logger::init();
    // let device = rodio::default_output_device().unwrap();
    // let sink = Sink::new(&device);
    // let mut decoder = Decoder::new(File::open("audio.mp3").unwrap());

    // let mut count = 0;
    // loop {
        // match decoder.next_frame() {
            // Ok(Frame { data, sample_rate, channels, .. }) => {
                // println!("Decoded {} samples {} {}", data.len(), sample_rate, channels);
                // let buff = SamplesBuffer::new(channels as u16, sample_rate as u32, data);
                // sink.append(buff);
                // println!("{}", count);
                // while count > 26 {
                    // // sleep and wait for rodio to drain a bit
                    // count = 0;
                    // thread::sleep(Duration::from_millis(10));
                // }
                // count += 1;
                // println!("finish append");
                // // thread::sleep(Duration::from_secs(1));
            // },
            // Err(Error::Eof) => break,
            // Err(e) => panic!("{:?}", e),
        // }
    // }
    // sink.sleep_until_end()

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    let backend = find(None).unwrap();

    let (player, _) = Player::new(move || (backend)(None));
    debug!("Playing...");
    player.load(&url, true);
    debug!("Done");
    loop {
        thread::sleep(Duration::from_secs(11111));
    }
}
