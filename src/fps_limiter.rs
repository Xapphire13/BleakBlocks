use macroquad::time::get_time;

pub struct FpsLimiter {
    target_fps: f64,
    last_frame_time: f64,
}

impl FpsLimiter {
    pub fn new(target_fps: f64) -> Self {
        Self {
            target_fps,
            last_frame_time: get_time(),
        }
    }

    pub fn wait_for_next_frame(&mut self) {
        let target_frame_time = 1.0 / self.target_fps;
        let current_time = get_time();
        let elapsed = current_time - self.last_frame_time;

        if elapsed < target_frame_time {
            let sleep_time = target_frame_time - elapsed;
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }

        self.last_frame_time = get_time();
    }
}
