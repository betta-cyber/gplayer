use std::io::prelude::*;
// use std::time::Duration;
// use super::player::PlayerCommand;
use futures::channel::oneshot::Sender;
use tempfile::NamedTempFile;
use reqwest::header::{CACHE_CONTROL, PRAGMA, HeaderMap, UPGRADE_INSECURE_REQUESTS, HOST, ACCEPT, ACCEPT_ENCODING, USER_AGENT, CONNECTION};
use reqwest::Method;
use reqwest::Proxy;


#[tokio::main]
pub async fn fetch_data(url: &str, buffer: NamedTempFile, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error>> {

    let mut headers = HeaderMap::new();
    headers.insert(CACHE_CONTROL, "no-cache".parse().unwrap());
    headers.insert(PRAGMA, "no-cache".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    headers.insert(HOST, "m7.music.126.net".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip,deflate,br".parse().unwrap());
    headers.insert(
        USER_AGENT,
        "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0".parse().unwrap(),
    );
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    println!("111");
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("socks5://127.0.0.1:8888").expect("proxy error"))
        .build().expect("builder error");
    println!("client {:#?}", client);
    let builder = client.request(Method::GET, url).headers(headers);
    println!("{:#?}", builder);
    let mut res = builder.send().await?;
    println!("{:#?}", res.status());
    println!("{:#?}", res.headers());
    let mut flag = true;
    let mut buffer = buffer;
    // send_msg(&filepath, tx);
    // println!("send msg");

    while let Some(chunk) = res.chunk().await? {
        // bytes
        info!("{:#?}", chunk);
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
