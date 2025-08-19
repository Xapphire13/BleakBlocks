use macroquad::{
    color::{Color, colors},
    math::Vec2,
    shapes::{draw_rectangle, draw_rectangle_lines},
};
use strum::EnumIter;

use crate::game::BACKGROUND_COLOR;

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
    pub velocity: Vec2,

    size: f32,
}

impl Block {
    pub fn new(position: Vec2, size: f32, block_type: BlockType) -> Self {
        Self {
            position,
            size,
            block_type,
            velocity: Vec2::ZERO,
        }
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
