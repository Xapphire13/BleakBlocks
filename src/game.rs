use std::collections::HashSet;

use macroquad::{
    color::Color,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Vec2,
    time::get_frame_time,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    block_renderer::render_blocks,
    constants::{layout::GRID_MARGIN, style::BACKGROUND_COLOR},
    coordinate::Coordinate,
    game_ui::GameUi,
    grid_layout::GridLayout,
    physics_system::{animate_blocks_falling, animate_columns_shifting},
    sprite_sheet::SpriteSheet,
};

#[derive(Clone)]
pub enum GameState {
    Playing,
    GameOver,
    BlocksFalling,
    ColumnsShifting,
}

pub struct Game {
    state: GameState,
    layout: GridLayout,
    sprite_sheet: SpriteSheet,
    ui: GameUi,
    score: u32,
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
            sprite_sheet: SpriteSheet::new(include_bytes!("../assets/sprites.png"), 2, 4, 50.0),
            ui: GameUi::new(),
            score: 0,
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
                        let blocks_removed = self.layout.remove_block_region(mouse_pos);
                        self.score += blocks_removed.pow(3);
                    }

                    if let Some(position) = self.layout.world_to_grid(mouse_pos) {
                        frame_state.hovered_blocks = self.layout.get_block_region(position)
                    };
                }
            }
            GameState::GameOver => {}
            GameState::BlocksFalling => {
                let animation_complete =
                    !animate_blocks_falling(&mut self.layout, get_frame_time());

                if animation_complete {
                    if self.layout.columns_need_shifting() {
                        self.state = GameState::ColumnsShifting;
                    } else {
                        self.state = GameState::Playing;
                    };
                }
            }
            GameState::ColumnsShifting => {
                let animation_complete =
                    !animate_columns_shifting(&mut self.layout, get_frame_time());

                if animation_complete {
                    self.state = GameState::Playing;
                };
            }
        }

        frame_state
    }

    pub fn render(&self, frame_state: FrameState) {
        clear_background(Color::from_hex(BACKGROUND_COLOR));

        render_blocks(&self.layout, &self.sprite_sheet, frame_state.hovered_blocks);
        self.ui.render(self);
    }

    fn is_game_over(&self) -> bool {
        self.layout.blocks_remaining == 0
    }

    pub fn state(&self) -> GameState {
        self.state.clone()
    }

    pub fn blocks_remaining(&self) -> u32 {
        self.layout.blocks_remaining
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
