use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

use macroquad::{prelude::*, rand::srand};

use crate::{fps_limiter::FpsLimiter, grid_layout::GridLayout};

mod block;
mod coordinate;
mod fps_limiter;
mod grid_layout;

const BACKGROUND_COLOR: u32 = 0x31263E;
const GRID_MARGIN: f32 = 20.0;
/// Gravity in pixels per second^2 that is applied to falling blocks
const GRAVITY: f32 = 2000.0;

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
    let mut game_state = GameState::Playing;
    let mut fps_limiter = FpsLimiter::new(60.0);
    let grid_size = screen_width().min(screen_height()) - 2. * GRID_MARGIN;
    let mut grid = GridLayout::new(
        Vec2::new(GRID_MARGIN, GRID_MARGIN),
        Vec2::new(grid_size, grid_size),
        10,
        10,
    );

    loop {
        clear_background(Color::from_hex(BACKGROUND_COLOR));

        // -----
        // Frame state
        // -----
        let mut hovered_blocks = HashSet::new();

        // -------------------
        // Handle player input
        // -------------------

        if let GameState::Playing = game_state {
            let mouse_pos = mouse_position().into();
            // Remove blocks when clicked
            if is_mouse_button_down(MouseButton::Left) {
                grid.remove_block_region(mouse_pos);
            }

            if let Some(position) = grid.world_to_grid(mouse_pos) {
                hovered_blocks = grid.get_block_region(position)
            };
        }

        // ---------
        // Rendering
        // ---------

        if let GameState::GameOver = game_state {
            let dimensions = draw_text("Game Over!", 0., 0., 32., BLANK);
            draw_text(
                "Game Over!",
                (screen_width() - dimensions.width) / 2.0,
                (screen_height() - dimensions.height) / 2.0,
                32.,
                WHITE,
            );
        } else {
            // Render grid with blocks
            grid.draw(hovered_blocks);
        }

        // ------
        // Update
        // ------
        match game_state {
            GameState::Playing => {
                if grid.is_game_over() {
                    game_state = GameState::GameOver;
                } else if grid.has_gaps() {
                    game_state = GameState::BlocksFalling(get_time());
                } else if grid.columns_need_shifting() {
                    game_state = GameState::ColumnsShifting(get_time());
                }
            }
            GameState::GameOver => {}
            GameState::BlocksFalling(last_update) => {
                let time_delta = get_time() - last_update;
                grid.animate_falling(time_delta);

                game_state = if grid.has_gaps() {
                    GameState::BlocksFalling(get_time())
                } else if grid.columns_need_shifting() {
                    GameState::ColumnsShifting(get_time())
                } else {
                    GameState::Playing
                };
            }
            GameState::ColumnsShifting(last_update) => {
                let time_delta = get_time() - last_update;
                grid.shift_columns(time_delta);

                game_state = if grid.columns_need_shifting() {
                    GameState::ColumnsShifting(get_time())
                } else {
                    GameState::Playing
                };
            }
        }

        fps_limiter.wait_for_next_frame();
        next_frame().await
    }
}

enum GameState {
    Playing,
    GameOver,
    /// Contains the time of the last update we made in this state
    BlocksFalling(f64),
    /// Contains the time of the last update we made in this state
    ColumnsShifting(f64),
}
