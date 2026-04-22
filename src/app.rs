use std::collections::HashSet;

use macroquad::{
    color::Color,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::{Vec2, vec2},
    time::get_frame_time,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    block::{Block, BlockState},
    constants::{
        physics::FORCE,
        style::{
            BACKGROUND_COLOR, BLOCK_INSET, BLOCK_SHADOW_FACTOR, EMPTY_BLOCK_COLOR,
            GRID_BACKGROUND_COLOR,
        },
        ui::{BLOCK_GAP, CONTAINER_INNER_PADDING, CORNER_RADIUS, WINDOW_PADDING},
    },
    coordinate::{Coordinate, coordinate},
    drawing::{draw_rounded_rect, draw_rounded_rect_asymmetric},
    game_session::{GameSession, GameState},
    game_ui::{ButtonId, GameUi},
    grid_layout::GridLayout,
    physics_system::PhysicsSystem,
    sprite_sheet::SpriteSheet,
};

#[derive(Copy, Clone, PartialEq)]
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
            state: app_state,
            sprite_sheet: SpriteSheet::new(include_bytes!("../assets/sprites.png"), 2, 4, 512.0),
            ui: GameUi::new(app_state),
            current_session: None,
        }
    }

    pub fn handle_input(&mut self) -> (InputEvent, FrameState) {
        let mut frame_state = FrameState::default();
        let mut input_event = InputEvent::None;

        if self.state == AppState::Playing {
            if let Some(session) = &self.current_session {
                if is_mouse_button_pressed(MouseButton::Left) {
                    input_event = InputEvent::BlockClicked(mouse_position().into());
                } else if let Some(position) = session.layout.world_to_grid(mouse_position().into())
                {
                    frame_state.hovered_blocks = session.layout.get_block_region(position);
                }
            }
        }

        if let Some(button_id) = self.ui.handle_input() {
            input_event = InputEvent::UIButton(button_id);
        }

        (input_event, frame_state)
    }

    pub fn update(&mut self, input: InputEvent) {
        self.ui
            .update_buttons(self.state, self.current_session.is_some());

        // macroquad has no resize event, so we recompute layout each frame
        if let Some(session) = &mut self.current_session {
            let panel_h = self.ui.status_panel_height();
            let (pos, dims) = compute_grid_rect(screen_width(), screen_height(), panel_h);
            session.layout.resize(pos, dims);
        }

        match input {
            InputEvent::BlockClicked(pos) => {
                if self.state == AppState::Playing {
                    if let Some(session) = &mut self.current_session {
                        let blocks_removed = session.layout.remove_block_region(pos);
                        session.score += App::calculate_points(blocks_removed);
                    }
                }
            }
            InputEvent::UIButton(button_id) => match button_id {
                ButtonId::Menu => self.set_state(AppState::MainMenu),
                ButtonId::NewGame => self.new_game(),
                ButtonId::Pause => self.set_state(AppState::MainMenu),
                ButtonId::Resume => self.set_state(AppState::Playing),
                _ => {}
            },
            InputEvent::None => {}
        }

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
                self.render_grid_background(session);
                self.render_blocks(session, frame_state.hovered_blocks);
            }
        }

        self.ui.render(UiContext {
            state: self.state,
            score: self.score(),
            blocks_remaining: self.blocks_remaining(),
        });
    }

    pub fn set_state(&mut self, state: AppState) {
        if self.state == AppState::GameOver && state == AppState::MainMenu {
            self.current_session = None;
        }

        self.state = state;
        self.ui
            .update_buttons(state, self.current_session.is_some());
    }

    fn render_grid_background(&self, session: &GameSession) {
        draw_rounded_rect(
            session.layout.x() - CONTAINER_INNER_PADDING,
            session.layout.y() - CONTAINER_INNER_PADDING,
            session.layout.width() + CONTAINER_INNER_PADDING * 2.0,
            session.layout.height() + CONTAINER_INNER_PADDING * 2.0,
            CORNER_RADIUS,
            GRID_BACKGROUND_COLOR,
        );
    }

    fn render_blocks(&self, session: &GameSession, hovered_blocks: HashSet<Coordinate>) {
        let block_size = session.layout.block_size;
        let half_gap = BLOCK_GAP / 2.0;
        let render_size = block_size - BLOCK_GAP;
        let cell_radius = (render_size * 0.15).min(6.0);

        // Pass 1: empty cell backgrounds for every cell
        for row in 0..session.layout.rows {
            for col in 0..session.layout.cols {
                let world_pos = session.layout.grid_to_world(coordinate(row, col));
                draw_rounded_rect(
                    world_pos.x + half_gap,
                    world_pos.y + half_gap,
                    render_size,
                    render_size,
                    cell_radius,
                    EMPTY_BLOCK_COLOR,
                );
            }
        }

        // Pass 2: block sprites on top
        for row in 0..session.layout.rows {
            for col in 0..session.layout.cols {
                let position = coordinate(row, col);
                if let Some(block) = session.layout.get_block(position) {
                    let block_state = if hovered_blocks.contains(&position) {
                        BlockState::Hover
                    } else {
                        BlockState::Default
                    };
                    let world_pos = session.layout.grid_to_world(position);
                    let anim_offset = session.physics_system.get_animation_offset(position);
                    self.render_block(
                        block,
                        block_state,
                        world_pos + vec2(half_gap, half_gap) + anim_offset,
                        render_size,
                    );
                }
            }
        }
    }

    fn render_block(&self, block: &Block, state: BlockState, position: Vec2, size: f32) {
        let darken = match state {
            BlockState::Default => 1.0,
            BlockState::Hover => 0.6,
        };
        let block_color = block.block_type.get_color();
        let shadow_color = Color::new(
            block_color.r * BLOCK_SHADOW_FACTOR * darken,
            block_color.g * BLOCK_SHADOW_FACTOR * darken,
            block_color.b * BLOCK_SHADOW_FACTOR * darken,
            1.0,
        );
        let fill_color = Color::new(
            block_color.r * darken,
            block_color.g * darken,
            block_color.b * darken,
            1.0,
        );
        let cell_radius = (size * 0.15).min(6.0);
        let inner_bottom_r = cell_radius * 1.1;

        draw_rounded_rect(
            position.x,
            position.y,
            size,
            size,
            cell_radius,
            shadow_color,
        );
        draw_rounded_rect_asymmetric(
            position.x,
            position.y,
            size,
            size - BLOCK_INSET,
            cell_radius,
            inner_bottom_r,
            fill_color,
        );
        self.sprite_sheet.render_sprite(
            block.block_type.get_sprite_id(),
            position,
            size,
            Color::new(darken, darken, darken, 1.0),
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
        let panel_h = self.ui.status_panel_height();
        let (pos, dims) = compute_grid_rect(screen_width(), screen_height(), panel_h);
        self.current_session = Some(GameSession {
            state: GameState::Playing,
            layout: GridLayout::new(pos, dims, 10, 10),
            score: 0,
            physics_system: PhysicsSystem::new(),
        });
        self.set_state(AppState::Playing);
    }
}

fn compute_grid_rect(screen_w: f32, screen_h: f32, status_panel_h: f32) -> (Vec2, Vec2) {
    let panel_y = screen_h - status_panel_h;
    let container_x = WINDOW_PADDING.x;
    let container_y = WINDOW_PADDING.y;
    let container_w = screen_w - WINDOW_PADDING.x * 2.0;
    let container_h = panel_y - WINDOW_PADDING.y - container_y;
    let grid_size = f32::min(
        container_w - CONTAINER_INNER_PADDING * 2.0,
        container_h - CONTAINER_INNER_PADDING * 2.0,
    );
    let grid_x = container_x + (container_w - grid_size) / 2.0;
    let grid_y = container_y + CONTAINER_INNER_PADDING;
    (Vec2::new(grid_x, grid_y), Vec2::new(grid_size, grid_size))
}

pub enum InputEvent {
    None,
    BlockClicked(Vec2),
    UIButton(ButtonId),
}

pub struct UiContext {
    pub state: AppState,
    pub score: u32,
    pub blocks_remaining: u32,
}

#[derive(Default)]
pub struct FrameState {
    hovered_blocks: HashSet<Coordinate>,
}
