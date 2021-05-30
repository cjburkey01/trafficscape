use std::time::Instant;

#[derive(Debug)]
pub struct TimeStep {
    // Timing
    last_time: Instant,
    delta_time: f64,

    // FPS tracking
    frame_count: u32,
    frame_time: f64,

    // UPS tracking
    tick_count: u32,
    tick_time: f64,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_count: 0,
            frame_time: 0.0,
            tick_count: 0,
            tick_time: 0.0,
        }
    }

    pub fn delta(&mut self) -> f64 {
        let current_time = Instant::now();
        let delta = current_time.duration_since(self.last_time).as_secs_f64();
        self.last_time = current_time;
        self.delta_time = delta;
        delta
    }

    pub fn frame_rate(&mut self) -> Option<u32> {
        // Increment frame counter
        self.frame_count += 1;
        self.frame_time += self.delta_time;

        // Update FPS counter every second
        if self.frame_time >= 1.0 {
            let tmp = self.frame_count;
            self.frame_count = 0;
            self.frame_time = 0.0;
            Some(tmp)
        } else {
            None
        }
    }
}
