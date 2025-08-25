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
    game_session::{GameSession, GameState},
    game_ui::{ButtonId, GameUi},
    grid_layout::GridLayout,
    physics_system::PhysicsSystem,
    sprite_sheet::SpriteSheet,
};

#[derive(Clone, PartialEq)]
pub enum AppState {
    Playing,
    GameOver,
    MainMenu,
}

pub struct App {
    state: AppState,
    sprite_sheet: SpriteSheet,
    ui: GameUi,
    current_session: Option<GameSession>,
}

impl App {
    pub fn new() -> Self {
        let app_state = AppState::MainMenu;
        Self {
            state: app_state.clone(),
            sprite_sheet: SpriteSheet::new(include_bytes!("../assets/sprites.png"), 2, 4, 45.0),
            ui: GameUi::new(app_state),
            current_session: None,
        }
    }

    pub fn handle_input(&mut self) -> FrameState {
        let mut frame_state = FrameState::default();

        if self.state == AppState::Playing {
            if let Some(session) = &mut self.current_session {
                if is_mouse_button_pressed(MouseButton::Left) {
                    // Remove blocks when clicked
                    let blocks_removed =
                        session.layout.remove_block_region(mouse_position().into());
                    session.score += App::calculate_points(blocks_removed);
                } else if let Some(position) = session.layout.world_to_grid(mouse_position().into())
                {
                    // Find hovered blocks
                    frame_state.hovered_blocks = session.layout.get_block_region(position)
                };
            }
        }

        if let Some(button_id) = self.ui.handle_input() {
            match button_id {
                ButtonId::Menu => {
                    self.set_state(AppState::MainMenu);
                }
                ButtonId::NewGame => {
                    self.new_game();
                }
                ButtonId::Resume => {
                    self.set_state(AppState::Playing);
                }
                _ => {}
            }
        }

        frame_state
    }

    pub fn update(&mut self) {
        if self.state == AppState::Playing {
            if let Some(session) = &mut self.current_session {
                match session.state {
                    GameState::Playing => {
                        if session.is_game_over() {
                            self.set_state(AppState::GameOver);
                        } else if let Some(falling_blocks) = session.layout.find_falling_blocks() {
                            falling_blocks.into_iter().for_each(|(from, to)| {
                                session.physics_system.queue_block_animation(from, to)
                            });
                            session.state = GameState::BlocksFalling;
                        } else if let Some(shifting_blocks) = session.layout.find_shifting_blocks()
                        {
                            shifting_blocks.into_iter().for_each(|(from, to)| {
                                session.physics_system.queue_block_animation(from, to)
                            });
                            session.state = GameState::ColumnsShifting;
                        }
                    }
                    GameState::BlocksFalling => {
                        let blocks_still_falling = session.physics_system.update(
                            &mut session.layout,
                            vec2(0.0, FORCE),
                            get_frame_time(),
                        );

                        if !blocks_still_falling {
                            if let Some(shifting_blocks) = session.layout.find_shifting_blocks() {
                                shifting_blocks.into_iter().for_each(|(from, to)| {
                                    session.physics_system.queue_block_animation(from, to)
                                });
                                session.state = GameState::ColumnsShifting;
                            } else {
                                session.state = GameState::Playing;
                            };
                        }
                    }
                    GameState::ColumnsShifting => {
                        let blocks_still_shifting = session.physics_system.update(
                            &mut session.layout,
                            vec2(-FORCE, 0.0),
                            get_frame_time(),
                        );

                        if !blocks_still_shifting {
                            session.state = GameState::Playing;
                        };
                    }
                }
            }
        }
    }

    pub fn render(&self, frame_state: FrameState) {
        clear_background(BACKGROUND_COLOR);

        if self.state == AppState::Playing {
            if let Some(session) = &self.current_session {
                self.render_grid(session);
                self.render_blocks(session, frame_state.hovered_blocks);
            }
        }

        self.ui.render(self);
    }

    pub fn state(&self) -> AppState {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: AppState) {
        if self.state == AppState::GameOver && state == AppState::MainMenu {
            self.current_session = None;
        }

        self.state = state.clone();
        self.ui
            .on_game_state_changed(self.state(), self.current_session.is_some());
    }

    fn render_grid(&self, session: &GameSession) {
        draw_rectangle(
            session.layout.x(),
            session.layout.y(),
            session.layout.width(),
            session.layout.height(),
            GRID_BACKGROUND_COLOR,
        );

        for col in 1..session.layout.cols {
            let x = session.layout.x() + session.layout.block_size * col as f32;
            draw_line(
                x,
                session.layout.y(),
                x,
                session.layout.y() + session.layout.height(),
                2.0,
                BACKGROUND_COLOR,
            );
        }

        for row in 1..session.layout.rows {
            let y = session.layout.y() + session.layout.block_size * row as f32;
            draw_line(
                session.layout.x(),
                y,
                session.layout.x() + session.layout.width(),
                y,
                2.0,
                BACKGROUND_COLOR,
            );
        }
    }

    fn render_blocks(&self, session: &GameSession, hovered_blocks: HashSet<Coordinate>) {
        // Render blocks
        for row in 0..session.layout.rows {
            for col in 0..session.layout.cols {
                let position = coordinate(row, col);

                if let Some(block) = session.layout.get_block(position) {
                    let block_state = if hovered_blocks.contains(&position) {
                        BlockState::Hover
                    } else {
                        BlockState::Default
                    };

                    self.render_block(
                        block,
                        block_state,
                        session.layout.grid_to_world(position)
                            + session.physics_system.get_animation_offset(position),
                    );
                }
            }
        }
    }

    fn render_block(&self, block: &Block, state: BlockState, position: Vec2) {
        self.sprite_sheet.render_sprite(
            block.block_type.get_sprite_id(),
            position,
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

    pub fn blocks_remaining(&self) -> u32 {
        self.current_session
            .as_ref()
            .map(|session| session.blocks_remaining())
            .unwrap_or(0)
    }

    pub fn score(&self) -> u32 {
        self.current_session
            .as_ref()
            .map(|session| session.score)
            .unwrap_or(0)
    }

    pub fn new_game(&mut self) {
        let grid_size = 450.0;

        self.current_session = Some(GameSession {
            state: GameState::Playing,
            layout: GridLayout::new(
                Vec2::new((screen_width() - grid_size) / 2.0, GRID_MARGIN),
                Vec2::new(grid_size, grid_size),
                10,
                10,
            ),
            score: 0,
            physics_system: PhysicsSystem::new(),
        });
        self.set_state(AppState::Playing);
    }
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
