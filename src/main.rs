use std::{sync::{atomic::{AtomicU64, Ordering::Relaxed}, mpsc::{channel, SendError, Sender}}, thread};


trait RateLimiter: Send + Sync {
    fn did_exceed(&self) -> bool;
    fn count_up(&self);
    fn notify(&self) -> Result<(), SendError<&str>>;
}

struct RateLimit<'a> {
    uid: &'a str,
    max: u64,
    count: AtomicU64,
    notifier: Sender<&'a str>
}

impl RateLimiter for RateLimit<'_> {
    fn count_up(&self) {
        self.count.fetch_add(1, Relaxed);
    }

    fn did_exceed(&self) -> bool {
        self.count.load(Relaxed) > self.max
    }

    fn notify(&self) -> Result<(), SendError<&str>>{
        self.notifier.send(self.uid)?;
        Ok(())
    }
}


fn send_and_sync(safe: impl RateLimiter) {
    safe.notify().unwrap();
}

fn main() {
    let (sender, receiver) = channel::<&str>();

    let limit = RateLimit{
        uid: "1234",
        count: AtomicU64::new(0),
        max: 200,
        notifier: sender.clone()
    };

    thread::spawn(move || {
        send_and_sync(limit);
    });

    println!("lets notify user[id: {}] about the rate limit!", receiver.recv().unwrap());
}