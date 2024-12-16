use std::sync::Arc;
use multithreaded_async::http_client::HttpClient;
use tokio::sync::mpsc::channel;
use async_std_resolver::{config, resolver};
use tokio::time::{Duration, timeout};

static BUFF_SIZE: u8 = 1;

#[tokio::main]
async fn main() {
    let resolver = resolver(
        config::ResolverConfig::default(),
        config::ResolverOpts::default(),
    ).await;
    let client = Arc::new(HttpClient::new(resolver));

    let urls = vec![
        "https://httpbin.org/headers",
        "https://google.com",
        "https://simpaix.net"
    ];

    let (send, mut recv) = channel::<u8>(BUFF_SIZE.into());
    for url_ in urls {
        let client_ = client.clone();
        let signal_ = send.clone();

        tokio::spawn(async move {
            let res = timeout(
                Duration::from_secs(10), 
                client_.http_get(url_)
            ).await.unwrap();

            match res {
                Ok(res) => println!("{:?}", res),
                Err(err) => println!("{:?}", err.to_string())
            }
        
            let _ = signal_.send(1).await;
        });
    }

    // receive all signals, then exit
    for _ in 0..3 {
        recv.recv().await;
    }
}