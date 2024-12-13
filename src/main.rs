use std::{
    sync::atomic::{AtomicU64, Ordering::SeqCst}, 
    sync::Arc, 
};
use tokio::sync::mpsc::{
    channel,
    Sender,
};

use tokio::time::{sleep, Duration};

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