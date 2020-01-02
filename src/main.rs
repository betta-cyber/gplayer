extern crate rodio;
extern crate reqwest;
extern crate tempfile;

use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use futures::channel::mpsc;
use tempfile::NamedTempFile;

use std::fs::File;

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


    let url = "http://mm1.doubanio.com/202001021709/513ecfaa7c3130c48e496711c0ea3aac/view/musicianmp3/mp3/x19543159.mp3";
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
