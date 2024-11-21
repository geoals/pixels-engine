use std::time::{Duration, Instant};

pub struct FpsCounter {
    frame_interval: Duration,
    last_print: Instant,
    frame_times: Vec<Duration>,
}

impl FpsCounter {
    pub fn new(target_fps: u64) -> Self {
        Self {
            frame_interval: Duration::from_micros(1_000_000 / target_fps),
            last_print: Instant::now(),
            frame_times: Vec::with_capacity(60),
        }
    }

    pub fn update_and_print(&mut self, frame_time: Duration) {
        self.frame_times.push(frame_time);

        if self.last_print.elapsed() >= Duration::from_millis(2000) {
            let avg_time =
                self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
            let max_time = self.frame_times.iter().max().unwrap();
            let min_time = self.frame_times.iter().min().unwrap();
            let fps = 1.0 / avg_time.as_secs_f32();

            println!("FPS Stats:");
            println!("  Average: {:.1} fps ({:?}/frame)", fps, avg_time);
            println!("  Min time: {:?}", min_time);
            println!("  Max time: {:?}", max_time);

            self.last_print = Instant::now();
            self.frame_times.clear();
        }
    }

    pub fn calculate_sleep_time(&self, elapsed_time: Duration) -> Duration {
        if elapsed_time < self.frame_interval {
            self.frame_interval - elapsed_time
        } else {
            Duration::ZERO
        }
    }
}
