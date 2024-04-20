use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct stopwatch_var {
    refresh_rate_msec: i32,
    counter_rate_msec: i32,
    counts_per_sec: i32,
    counts_per_min: i32,
    counts_per_hour: i32,
    count: i32,
}

fn counter(is_running: Arc<AtomicBool>) {
    let mut count = 0;
    while is_running.load(Ordering::Relaxed) {
        println!("count {count} in counter");
        count = count + 1;
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    let stop = Arc::new(AtomicBool::new(true));
    let stop_me = stop.clone();
    thread::spawn(move || counter(stop_me));

    for i in 1..5 {
        println!("main thread! {}", i);
        thread::sleep(Duration::from_millis(10));
    }
    stop.store(false, Ordering::Release);
    println!("end main");
}
