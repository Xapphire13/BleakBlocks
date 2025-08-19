use std::collections::HashSet;

use macroquad::{
    color::{Color, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
    text::{draw_text, measure_text},
    time::get_time,
    window::{clear_background, screen_height, screen_width},
};

use crate::{GameState, coordinate::Coordinate, grid_layout::GridLayout};

pub const BACKGROUND_COLOR: u32 = 0x31263E;
const GRID_MARGIN: f32 = 20.0;

pub struct Game {
    state: GameState,
    layout: GridLayout,
}

impl Game {
    pub fn new() -> Self {
        let grid_size = screen_width().min(screen_height()) - 2. * GRID_MARGIN;

        Self {
            state: GameState::Playing,
            layout: GridLayout::new(
                Vec2::new(GRID_MARGIN, GRID_MARGIN),
                Vec2::new(grid_size, grid_size),
                10,
                10,
            ),
        }
    }

    pub fn update(&mut self) -> FrameState {
        let mut frame_state = FrameState::default();

        match self.state {
            GameState::Playing => {
                if self.layout.is_game_over() {
                    self.state = GameState::GameOver;
                } else if self.layout.has_gaps() {
                    self.state = GameState::BlocksFalling(get_time());
                } else if self.layout.columns_need_shifting() {
                    self.state = GameState::ColumnsShifting(get_time());
                } else {
                    let mouse_pos = mouse_position().into();
                    // Remove blocks when clicked
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.layout.remove_block_region(mouse_pos);
                    }

                    if let Some(position) = self.layout.world_to_grid(mouse_pos) {
                        frame_state.hovered_blocks = self.layout.get_block_region(position)
                    };
                }
            }
            GameState::GameOver => {}
            GameState::BlocksFalling(last_update) => {
                let time_delta = get_time() - last_update;
                self.layout.animate_falling(time_delta);

                self.state = if self.layout.has_gaps() {
                    GameState::BlocksFalling(get_time())
                } else if self.layout.columns_need_shifting() {
                    GameState::ColumnsShifting(get_time())
                } else {
                    GameState::Playing
                };
            }
            GameState::ColumnsShifting(last_update) => {
                let time_delta = get_time() - last_update;
                self.layout.shift_columns(time_delta);

                self.state = if self.layout.columns_need_shifting() {
                    GameState::ColumnsShifting(get_time())
                } else {
                    GameState::Playing
                };
            }
        }

        frame_state
    }

    pub fn render(&self, frame_state: FrameState) {
        clear_background(Color::from_hex(BACKGROUND_COLOR));

        if let GameState::GameOver = self.state {
            let dimensions = measure_text("Game Over!", None, 32, 1.);
            draw_text(
                "Game Over!",
                (screen_width() - dimensions.width) / 2.0,
                (screen_height() - dimensions.height) / 2.0,
                32.,
                WHITE,
            );
        } else {
            // Render grid with blocks
            self.layout.draw(frame_state.hovered_blocks);
        }
    }
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
