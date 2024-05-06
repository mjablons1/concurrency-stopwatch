use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct StopWatch {
    is_running: Arc<AtomicBool>,
    refresh_rate_msec: Arc<AtomicU64>,
    counter_rate_msec: Arc<AtomicU64>,
    counts_per_sec: i32,
    counts_per_min: i32,
    counts_per_hour: i32,
    count: i32,
}

impl StopWatch {
    fn new() -> StopWatch {
        let mut sw = StopWatch {
            is_running: Arc::new(AtomicBool::new(true)),
            refresh_rate_msec: Arc::new(AtomicU64::new(1000)),
            counter_rate_msec: Arc::new(AtomicU64::new(100)),
            counts_per_sec: 0,
            counts_per_min: 0,
            counts_per_hour: 0,
            count: 0,
        };
        StopWatch::set_counts(sw)
    }

    fn set_counts(mut sw: StopWatch) -> StopWatch {
        sw.counts_per_sec = (1000 / sw.counter_rate_msec.load(Ordering::Relaxed)) as i32;
        sw.counts_per_min = 60 * sw.counts_per_sec;
        sw.counts_per_hour = 3600 * sw.counts_per_sec;
        return sw;
    }

    pub fn count_to_time(&mut self) -> String {
        let hours = (self.count / self.counts_per_hour).to_string();
        let reminder = self.count % self.counts_per_hour;
        let minutes = (reminder / self.counts_per_min).to_string();
        let reminder2 = reminder % self.counts_per_min;
        let seconds = (reminder2 / self.counts_per_sec).to_string();
        let milisec = (reminder2 % self.counts_per_sec);
        hours + ":" + &minutes + ":" + &seconds + ":" + &milisec.to_string()
    }

    pub fn counter(&mut self) {
        let is_running_me = self.is_running.clone();
        let counter_rate_msec = self.counter_rate_msec.clone();

        thread::spawn(move || {
            while is_running_me.load(Ordering::Relaxed) {
                println!("count {}", counter_rate_msec.load(Ordering::Relaxed));
                thread::sleep(Duration::from_millis(
                    counter_rate_msec.load(Ordering::Relaxed),
                ));
            }
        });
    }

    pub fn refresh(&mut self) {
        let is_running_me = self.is_running.clone();
        let refresh_rate_msec = self.refresh_rate_msec.clone();

        thread::spawn(move || {
            while is_running_me.load(Ordering::Relaxed) {
                println!("refresh");
                thread::sleep(Duration::from_millis(
                    refresh_rate_msec.load(Ordering::Relaxed),
                ));
            }
        });
    }
}

fn main() {
    let mut sw = StopWatch::new();
    sw.counter();
    sw.refresh();
    for i in 1..5 {
        println!("main thread! {}", i);
        thread::sleep(Duration::from_millis(1000));
    }
    sw.is_running.store(false, Ordering::Relaxed);
    println!("end main");
}
