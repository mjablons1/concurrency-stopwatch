use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct StopWatch {
    is_running: Arc<AtomicBool>,
    refresh_rate_msec: i32,
    counter_rate_msec: Arc<AtomicI32>,
    counts_per_sec: i32,
    counts_per_min: i32,
    counts_per_hour: i32,
    count: i32,
}

impl StopWatch {
    fn new() -> StopWatch {
        let mut new_sw = StopWatch {
            is_running: Arc::new(AtomicBool::new(true)),
            refresh_rate_msec: 1000,
            counter_rate_msec: Arc::new(AtomicI32::new(100)),
            counts_per_sec: 0,
            counts_per_min: 0,
            counts_per_hour: 0,
            count: 0,
        };
        StopWatch::set_counts(new_sw)
    }

    fn set_counts(mut sw: StopWatch) -> StopWatch {
        sw.counts_per_sec = 1000 / sw.counter_rate_msec.load(Ordering::Relaxed);
        sw.counts_per_min = 60 * sw.counts_per_sec;
        sw.counts_per_hour = 3600 * sw.counts_per_sec;
        return sw;
    }

    pub fn counter(&mut self) {
        let is_running_me = self.is_running.clone();
        let counter_rate_msec = self.counter_rate_msec.clone();

        thread::spawn(move || {
            while is_running_me.load(Ordering::Relaxed) {
                println!("count in counter");
                thread::sleep(Duration::from_millis(
                    counter_rate_msec.load(Ordering::Relaxed) as u64,
                ));
            }
        });
    }
}

fn main() {
    let mut sw = StopWatch::new();
    sw.counter();
    for i in 1..5 {
        println!("main thread! {}", i);
        thread::sleep(Duration::from_millis(1000));
    }
    sw.is_running.store(false, Ordering::Relaxed);
    println!("end main");
}
