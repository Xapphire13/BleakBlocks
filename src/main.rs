use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::{prelude::*, rand::srand};

use crate::{fps_limiter::FpsLimiter, game::Game};

mod block;
mod coordinate;
mod fps_limiter;
mod game;
mod grid_layout;
mod physics_system;

fn window_conf() -> Conf {
    Conf {
        window_title: "Bleak Blocks".to_owned(),
        window_height: 500,
        window_width: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Seed the random number generator based on system time
    srand(
        (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % u64::MAX as u128) as u64,
    );
    let mut fps_limiter = FpsLimiter::new(60.0);
    let mut game = Game::new();

    loop {
        let frame_state = game.update();
        game.render(frame_state);

        fps_limiter.wait_for_next_frame();
        next_frame().await
    }
}
