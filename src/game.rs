use std::collections::HashSet;

use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::{Vec2, vec2},
    shapes::{draw_line, draw_rectangle},
    time::get_frame_time,
    window::{clear_background, screen_width},
};

use crate::{
    block::{Block, BlockState},
    constants::{
        layout::GRID_MARGIN,
        physics::FORCE,
        style::{BACKGROUND_COLOR, GRID_BACKGROUND_COLOR},
    },
    coordinate::{Coordinate, coordinate},
    game_ui::GameUi,
    grid_layout::GridLayout,
    physics_system::PhysicsSystem,
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
    physics_system: PhysicsSystem,
}

impl Game {
    pub fn new() -> Self {
        let grid_size = 450.0;

        Self {
            state: GameState::Playing,
            layout: GridLayout::new(
                Vec2::new((screen_width() - grid_size) / 2.0, GRID_MARGIN),
                Vec2::new(grid_size, grid_size),
                10,
                10,
            ),
            sprite_sheet: SpriteSheet::new(include_bytes!("../assets/sprites.png"), 2, 4, 45.0),
            ui: GameUi::new(),
            score: 0,
            physics_system: PhysicsSystem::new(),
        }
    }

    pub fn update(&mut self) -> FrameState {
        let mut frame_state = FrameState::default();

        match self.state {
            GameState::Playing => {
                if self.is_game_over() {
                    self.state = GameState::GameOver;
                } else if let Some(falling_blocks) = self.layout.find_falling_blocks() {
                    falling_blocks
                        .into_iter()
                        .for_each(|(from, to)| self.physics_system.track_block(from, to));
                    self.state = GameState::BlocksFalling;
                } else if let Some(shifting_blocks) = self.layout.find_shifting_blocks() {
                    shifting_blocks
                        .into_iter()
                        .for_each(|(from, to)| self.physics_system.track_block(from, to));
                    self.state = GameState::ColumnsShifting;
                } else {
                    let mouse_pos = mouse_position().into();
                    // Remove blocks when clicked
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let blocks_removed = self.layout.remove_block_region(mouse_pos);
                        self.score += Game::calculate_points(blocks_removed);
                    }

                    if let Some(position) = self.layout.world_to_grid(mouse_pos) {
                        frame_state.hovered_blocks = self.layout.get_block_region(position)
                    };
                }
            }
            GameState::GameOver => {}
            GameState::BlocksFalling => {
                let blocks_still_falling = self.physics_system.update(
                    &mut self.layout,
                    vec2(0.0, FORCE),
                    get_frame_time(),
                );

                if !blocks_still_falling {
                    if let Some(shifting_blocks) = self.layout.find_shifting_blocks() {
                        shifting_blocks
                            .into_iter()
                            .for_each(|(from, to)| self.physics_system.track_block(from, to));
                        self.state = GameState::ColumnsShifting;
                    } else {
                        self.state = GameState::Playing;
                    };
                }
            }
            GameState::ColumnsShifting => {
                let blocks_still_shifting = self.physics_system.update(
                    &mut self.layout,
                    vec2(-FORCE, 0.0),
                    get_frame_time(),
                );

                if !blocks_still_shifting {
                    self.state = GameState::Playing;
                };
            }
        }

        frame_state
    }

    pub fn render(&self, frame_state: FrameState) {
        clear_background(BACKGROUND_COLOR);

        if !matches!(self.state, GameState::GameOver) {
            self.render_grid();
            self.render_blocks(frame_state.hovered_blocks);
        }

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

    fn render_grid(&self) {
        draw_rectangle(
            self.layout.position.x,
            self.layout.position.y,
            self.layout.dimensions.x,
            self.layout.dimensions.y,
            GRID_BACKGROUND_COLOR,
        );

        for col in 1..self.layout.cols {
            let x = self.layout.position.x + self.layout.block_size * col as f32;
            draw_line(
                x,
                self.layout.position.y,
                x,
                self.layout.position.y + self.layout.dimensions.y,
                2.0,
                BACKGROUND_COLOR,
            );
        }

        for row in 1..self.layout.rows {
            let y = self.layout.position.y + self.layout.block_size * row as f32;
            draw_line(
                self.layout.position.x,
                y,
                self.layout.position.x + self.layout.dimensions.x,
                y,
                2.0,
                BACKGROUND_COLOR,
            );
        }
    }

    fn render_blocks(&self, hovered_blocks: HashSet<Coordinate>) {
        // Render blocks
        for row in 0..self.layout.rows {
            for col in 0..self.layout.cols {
                let position = coordinate(row, col);
                if let Some(block) = self.layout.get_block_at_grid_position(position) {
                    let block_state = if hovered_blocks.contains(&position) {
                        BlockState::Hover
                    } else {
                        BlockState::Default
                    };

                    self.render_block(block, block_state);
                }
            }
        }
    }

    fn render_block(&self, block: &Block, state: BlockState) {
        self.sprite_sheet.render_sprite(
            block.block_type.get_sprite_id(),
            block.position,
            block.size,
            match state {
                BlockState::Default => 1.0,
                BlockState::Hover => 0.5,
            },
        );
    }

    /// Calculate points using (n-1)^2 formula
    fn calculate_points(number_of_blocks: u32) -> u32 {
        (number_of_blocks.saturating_sub(1)).pow(2)
    }
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
