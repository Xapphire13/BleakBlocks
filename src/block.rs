use macroquad::{
    color::{Color, colors},
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
};
use strum::EnumIter;

use crate::{BACKGROUND_COLOR, GRAVITY};

pub enum BlockState {
    Default,
    Hover,
}

#[derive(EnumIter, Clone, Eq, PartialEq, Hash)]
pub enum BlockType {
    Potion,
    Blood,
    Ghost,
    Poison,
    Coffin,
    Gravestone,
    Flame,
    Brain,
}

impl BlockType {
    fn get_color(&self) -> Color {
        match self {
            BlockType::Potion => colors::GREEN,
            BlockType::Blood => colors::RED,
            BlockType::Ghost => colors::WHITE,
            BlockType::Poison => colors::PURPLE,
            BlockType::Coffin => colors::BLACK,
            BlockType::Flame => colors::ORANGE,
            BlockType::Gravestone => colors::DARKGRAY,
            BlockType::Brain => colors::MAGENTA,
        }
    }
}

pub struct Block {
    pub block_type: BlockType,
    pub position: Vec2,

    size: f32,
    velocity: f32,
}

impl Block {
    pub fn new(position: Vec2, size: f32, block_type: BlockType) -> Self {
        Self {
            position,
            size,
            block_type,
            velocity: 0.0,
        }
    }

    pub fn apply_gravity(&mut self, elapsed_time: f64) {
        self.position.y += (self.velocity as f64 * elapsed_time) as f32;
        self.velocity += (GRAVITY as f64 * elapsed_time) as f32;
    }

    /// Similar to falling, but for shifting columns to the left
    pub fn apply_gravity_left(&mut self, elapsed_time: f64) {
        self.position.x -= (self.velocity as f64 * elapsed_time) as f32;
        self.velocity += (GRAVITY as f64 * elapsed_time) as f32;
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }

    pub fn draw(&self, state: BlockState) {
        let color = match state {
            BlockState::Default => self.block_type.get_color(),
            BlockState::Hover => colors::LIGHTGRAY,
        };

        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            color,
        );

        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            1.0,
            Color::from_hex(BACKGROUND_COLOR),
        );
    }
}
