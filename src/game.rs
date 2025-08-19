use std::collections::HashSet;

use macroquad::{
    color::{Color, WHITE},
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
    text::{draw_text, measure_text},
    time::get_frame_time,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    block_renderer::render_blocks,
    constants::{BACKGROUND_COLOR, GRID_MARGIN},
    coordinate::Coordinate,
    grid_layout::GridLayout,
    physics_system::{animate_blocks_falling, animate_columns_shifting},
};

enum GameState {
    Playing,
    GameOver,
    BlocksFalling,
    ColumnsShifting,
}

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
                if self.is_game_over() {
                    self.state = GameState::GameOver;
                } else if self.layout.has_gaps() {
                    self.state = GameState::BlocksFalling;
                } else if self.layout.columns_need_shifting() {
                    self.state = GameState::ColumnsShifting;
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
            GameState::BlocksFalling => {
                animate_blocks_falling(&mut self.layout, get_frame_time());

                self.state = if self.layout.has_gaps() {
                    GameState::BlocksFalling
                } else if self.layout.columns_need_shifting() {
                    GameState::ColumnsShifting
                } else {
                    GameState::Playing
                };
            }
            GameState::ColumnsShifting => {
                animate_columns_shifting(&mut self.layout, get_frame_time());

                self.state = if self.layout.columns_need_shifting() {
                    GameState::ColumnsShifting
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
            render_blocks(&self.layout, frame_state.hovered_blocks);
        }
    }

    fn is_game_over(&self) -> bool {
        self.layout.blocks_remaining == 0
    }
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
