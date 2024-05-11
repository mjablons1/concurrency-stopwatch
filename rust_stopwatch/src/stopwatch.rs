use std::io::stdin;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn count_to_time(
    count: i32,
    counts_per_hour: i32,
    counts_per_min: i32,
    counts_per_sec: i32,
) -> String {
    let hours = (count / counts_per_hour).to_string();
    let reminder = count % counts_per_hour;
    let minutes = (reminder / counts_per_min).to_string();
    let reminder2 = reminder % counts_per_min;
    let seconds = (reminder2 / counts_per_sec).to_string();
    let milisec = reminder2 % counts_per_sec;
    hours + ":" + &minutes + ":" + &seconds + ":" + &milisec.to_string()
}
struct StopWatch {
    is_running: Arc<AtomicBool>,
    refresh_rate_msec: Arc<AtomicU64>,
    counter_rate_msec: Arc<AtomicU64>,
    counts_per_sec: i32,
    counts_per_min: i32,
    counts_per_hour: i32,
    count: Arc<AtomicI32>,
}

impl StopWatch {
    fn new() -> StopWatch {
        let sw = StopWatch {
            is_running: Arc::new(AtomicBool::new(true)),
            refresh_rate_msec: Arc::new(AtomicU64::new(1000)),
            counter_rate_msec: Arc::new(AtomicU64::new(100)),
            counts_per_sec: 0,
            counts_per_min: 0,
            counts_per_hour: 0,
            count: Arc::new(AtomicI32::new(0)),
        };
        StopWatch::set_counts(sw)
    }

    fn set_counts(mut sw: StopWatch) -> StopWatch {
        sw.counts_per_sec = (1000 / sw.counter_rate_msec.load(Ordering::Relaxed)) as i32;
        sw.counts_per_min = 60 * sw.counts_per_sec;
        sw.counts_per_hour = 3600 * sw.counts_per_sec;
        return sw;
    }

    pub fn refresh(&mut self) {
        let is_running_me = self.is_running.clone();
        let refresh_rate_msec = self.refresh_rate_msec.clone();
        let count = self.count.clone();
        let loc_counts_per_hour = self.counts_per_hour;
        let loc_counts_per_min = self.counts_per_min;
        let loc_counts_per_sec = self.counts_per_sec;

        thread::spawn(move || {
            while is_running_me.load(Ordering::Relaxed) {
                println!(
                    "{}",
                    count_to_time(
                        count.load(Ordering::Relaxed),
                        loc_counts_per_hour,
                        loc_counts_per_min,
                        loc_counts_per_sec
                    )
                );
                thread::sleep(Duration::from_millis(
                    refresh_rate_msec.load(Ordering::Relaxed),
                ));
            }
        });
    }

    pub fn counter(&mut self) {
        let is_running_me = self.is_running.clone();
        let count = self.count.clone();
        let counter_rate_msec = self.counter_rate_msec.clone();

        thread::spawn(move || {
            while is_running_me.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(
                    counter_rate_msec.load(Ordering::Relaxed),
                ));
                let inc_count = count.load(Ordering::Relaxed);
                count.store(inc_count + 1, Ordering::Relaxed);
            }
        });
    }

    pub fn read_input(&mut self) {
        let mut resp = String::new();
        loop {
            match stdin().read_line(&mut resp) {
                Ok(_) => {
                    if resp.eq("q\n") {
                        self.is_running.store(false, Ordering::Relaxed);
                        break;
                    } else if resp.eq("r\n") {
                        self.count.store(0, Ordering::Relaxed);
                    }
                    resp = "".to_string();
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }
}

fn main() {
    let mut sw = StopWatch::new();
    sw.refresh();
    sw.counter();
    sw.read_input();
}
