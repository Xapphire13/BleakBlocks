use macroquad::time::get_time;

pub struct FpsLimiter {
    target_fps: f64,
    frame_deadline: f64,
}

impl FpsLimiter {
    pub fn new(target_fps: f64) -> Self {
        Self {
            target_fps,
            frame_deadline: get_time(),
        }
    }

    pub fn wait_for_next_frame(&mut self) {
        let frame_duration = 1.0 / self.target_fps;
        self.frame_deadline += frame_duration;

        let now = get_time();
        if now < self.frame_deadline {
            std::thread::sleep(std::time::Duration::from_secs_f64(
                self.frame_deadline - now,
            ));
        }

        // Reset if a frame genuinely ran long, to avoid a burst of zero-sleep catch-up frames
        let now = get_time();
        if self.frame_deadline < now - frame_duration {
            self.frame_deadline = now;
        }
    }
}
