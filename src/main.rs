use std::{rc::Rc, sync::atomic::{AtomicU64, Ordering::Relaxed}};


trait RateLimiter {
    fn did_exceed(&self) -> bool;
    fn count_up(&self);
}

struct RateLimit<'a> {
    uid: &'a str,
    count: AtomicU64,
    max: u64
}

impl RateLimiter for RateLimit<'_> {
    fn count_up(&self) {
        self.count.fetch_add(1, Relaxed);
    }

    fn did_exceed(&self) -> bool {
        self.count.load(Relaxed) > self.max
    }
}

fn send_and_sync(safe: impl RateLimiter + Send + Sync + 'static) {}

fn main() {
    let limit = RateLimit{
        uid: "",
        count: AtomicU64::new(0),
        max: 200
    };

    send_and_sync(limit);
}