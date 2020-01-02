extern crate rodio;
extern crate reqwest;
extern crate tempfile;

mod player;

use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use futures::channel::mpsc;
use tempfile::NamedTempFile;
use std::env;


enum PlayerCommand {
    Load(String),
    Play,
    Pause,
    Stop,
    Seek(u32),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let (tx, mut rx) = mpsc::unbounded::<PlayerCommand>();
    // let mut buffer = File::create("foo").unwrap();

    thread::spawn(move || {
        println!("Hello from a thread!");
        loop {
            let a = rx.try_next();
            match a {
                Ok(v) => {
                    match v {
                        Some(v) => {
                            match v {
                                PlayerCommand::Load(path) => {
                                    let b = std::fs::File::open(&path).unwrap();
                                    let source = rodio::Decoder::new(
                                        BufReader::with_capacity(100, b)
                                    ).unwrap();
                                    sink.append(source);

                                    sink.sleep_until_end();
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
                Err(_) => {}
            }
        }
    });

    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    // let url = "http://mr1.doubanio.com/993e6b86e35bb933e6d8bfedab660db7/0/fm/song/p1584951_64k.mp3";
    let mut res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    let mut flag = true;

    // let path = "foo";
    let mut buffer = NamedTempFile::new()?;
    // let mut buffer = File::create(path).unwrap();
    while let Some(chunk) = res.chunk().await? {
        // bytes
        buffer.write(&chunk[..]).unwrap();
        if flag {
            tx.unbounded_send(PlayerCommand::Load(buffer.path().to_string_lossy().to_string())).expect("send error");
            flag = false;
        }
    }
    println!("finish");

    // const CHUNK_SIZE: u32 = 10240;
    loop {
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
