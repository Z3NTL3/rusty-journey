use std::{
    sync::atomic::{AtomicU64, Ordering::SeqCst}, 
    sync::Arc, 
};
use multithreaded_async::http_client::{self, HttpClient};
use tokio::sync::mpsc::{
    channel,
    Sender,
};

use tokio::time::{sleep, Duration, timeout};
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, Resolver};

struct Counter {
    count: AtomicU64,
    max: u64,
    limit_sig: Sender<String>
}

impl Counter {
    // must call limit_reached before counting up
    fn count_up(&self) {
        self.count.fetch_add(1, SeqCst);
    }

    async fn limit_reached(&self) -> bool {
        sleep(Duration::from_millis(100)).await; // just to give control back to the tokio runtime
        // this async fn is useless i know
        self.count.load(SeqCst) >= self.max
    }
}

#[tokio::main]
async fn main() {
    let client = HttpClient::new();

    let res = timeout(
        Duration::from_millis(500), 
        client.http_get("44.196.3.45", "/headers", "httpbin.org")
    ).await.unwrap();

    match res {
        Ok(res) => {
            println!("{:?}", res)
        },
        Err(err) => println!("{}", err.to_string())
    }

    return;
    let (tx, mut rx) = channel::<String>(1);

    let counter = Arc::new(
        Counter{
            count: AtomicU64::new(0),
            max: 500,
            limit_sig: tx
        }
    );
    
    // spawn taks
    // the async runtime handles the rest simply said
    for _ in 0..501 {
        let c = counter.clone();
        tokio::spawn( async move {
            if !c.limit_reached().await {
                return c.count_up();
            }

            // will be an error after closing but thats fine to ignore
            let _ = c.limit_sig.send(String::from("limit reached")).await;
        });
    }
    
    if let Some(i) = rx.recv().await {
        println!("got = {} - counter at: {}", i, counter.count.load(SeqCst));

        drop(rx);
    }
    
}