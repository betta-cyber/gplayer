use std::io::prelude::*;
// use std::time::Duration;
// use super::player::PlayerCommand;
use futures::channel::oneshot::Sender;
use tempfile::NamedTempFile;
// use std::fs::File;
// use std::sync::Arc;

#[tokio::main]
pub async fn fetch_data(url: &str, buffer: NamedTempFile, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {

    let mut res = reqwest::get(url).await?;
    let mut flag = true;
    let mut buffer = buffer;
    // send_msg(&filepath, tx);
    // println!("send msg");

    while let Some(chunk) = res.chunk().await? {
        // bytes
        buffer.write(&chunk[..]).unwrap();
        if flag {
            flag = false;
        }
    }
    println!("finish download");
    Ok(())

}

fn send_msg(path: &str, tx: Sender<String>) {
    tx.send(path.to_owned()).expect("send error");
}
