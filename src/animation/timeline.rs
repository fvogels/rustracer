use super::{Duration, TimeStamp};

pub struct TimeLine {
    duration: Duration,
    frames_per_second: u32,
}

impl TimeLine {
    pub fn new(duration: Duration, frames_per_second: u32) -> Self {
        TimeLine { duration, frames_per_second }
    }

    pub fn iter(&self) -> impl Iterator<Item=(u32, TimeStamp)> {
        let total_frame_count = self.frame_count();

        {
            let duration = self.duration;
            let fps = self.frames_per_second;
            (0..total_frame_count).map(move |i| (i, TimeStamp::zero() + duration / fps as f64 * i as f64))
        }
    }

    pub fn frame_count(&self) -> u32 {
        (self.duration.in_seconds() * self.frames_per_second as f64) as u32
    }
}
