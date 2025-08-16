use std::collections::HashSet;

use macroquad::prelude::*;

use crate::{block::BlockState, fps_limiter::FpsLimiter, game_grid::GameGrid};

mod block;
mod fps_limiter;
mod game_grid;
mod has_bounds;

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
    let mut game_state = GameState::Playing;
    let mut fps_limiter = FpsLimiter::new(60.0);
    let grid_size = screen_width().min(screen_height()) - 2. * GRID_MARGIN;
    let mut grid = GameGrid::new(GRID_MARGIN, GRID_MARGIN, grid_size, grid_size, 10, 10);

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
            let (mouse_x, mouse_y) = mouse_position();
            // Remove blocks when clicked
            if is_mouse_button_down(MouseButton::Left) {
                grid.remove_block_region(mouse_x, mouse_y);
            }

            if let Some(block) = grid.get_block_at_pixel_position(mouse_x, mouse_y) {
                hovered_blocks = grid.get_block_region(block)
            };
        }

        // ---------
        // Rendering
        // ---------

        // Render blocks
        for block in grid.blocks.iter().flatten() {
            if let Some(block) = block.as_ref() {
                let block_state = if hovered_blocks.contains(block) {
                    BlockState::Hover
                } else {
                    BlockState::Default
                };
                block.draw(block_state);
            }
        }

        // Render grid on top of blocks
        grid.draw();

        // ------
        // Update
        // ------
        match game_state {
            GameState::Playing => {
                if grid.has_gaps() {
                    game_state = GameState::BlocksFalling(get_time());
                }
            }
            GameState::BlocksFalling(last_update) => {
                let time_delta = get_time() - last_update;
                grid.animate_falling(time_delta);

                game_state = if grid.has_gaps() {
                    GameState::BlocksFalling(get_time())
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
    /// Contains the time of the last update we made in this state
    BlocksFalling(f64),
}
