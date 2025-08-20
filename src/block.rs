use macroquad::math::Vec2;
use strum::EnumIter;

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

pub struct Block {
    pub block_type: BlockType,
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
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
}
