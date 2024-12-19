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
    ];
    let tasks = urls.len();

    let (send, mut recv) = channel::<u8>(BUFF_SIZE.into());
    for url_ in urls {
        let client_ = client.clone();
        let signal_ = send.clone();

        tokio::spawn(async move {
            let res = timeout(
                Duration::from_secs(10), 
                client_.https_get(url_)
            ).await.unwrap();

            match res {
                Ok(res) => println!("{:?}", res),
                Err(err) => println!("{:?}", err.to_string())
            }
        
            let _ = signal_.send(1).await;
        });
    }

    // receive all signals, then exit
    for _ in 0..tasks {
        recv.recv().await;
    }
}

/*
This code below is just to learn
 */
#[cfg(test)]
#[tokio::main]
async fn test_future(){
    use std::{future::Future, net::TcpStream, pin::Pin, task::{Context, Poll::{self, Pending, Ready}}};

    use tokio::sync::Mutex;

    #[derive(Clone)]
    struct Socket {
        addr: &'static str,
        result: Arc<Mutex<Result<TcpStream, Box<dyn std::error::Error + Send+ Sync + 'static>>>>
    }

    impl Future for Socket {
        type Output = Result<TcpStream, Box<dyn std::error::Error + Send+ Sync + 'static>>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let waker = cx.waker().clone();

            let sock =  Arc::clone(&self.result);
            let addr = self.addr;

            tokio::spawn(async move {
                let conn = TcpStream::connect(addr);

                let mut instance = sock.lock().await;
                match conn {
                    Ok(stream) => {
                        *instance = Ok(stream);
                    },
                    Err(err) => {
                        *instance = Err(Box::new(err));
                    }
                }

                waker.wake_by_ref();
            });

            // todo stuff error prone, my intention is first to make it actually compile
            Pending
        }

    }
}