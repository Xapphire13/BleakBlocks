use macroquad::math::Vec2;
use strum::EnumIter;

use crate::sprite_sheet::SpriteId;

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
    pub fn get_sprite_id(&self) -> SpriteId {
        match self {
            BlockType::Brain => SpriteId(0, 0),
            BlockType::Blood => SpriteId(0, 1),
            BlockType::Poison => SpriteId(0, 2),
            BlockType::Ghost => SpriteId(0, 3),
            BlockType::Gravestone => SpriteId(1, 0),
            BlockType::Flame => SpriteId(1, 1),
            BlockType::Potion => SpriteId(1, 2),
            BlockType::Coffin => SpriteId(1, 3),
        }
    }
}

pub struct Block {
    pub block_type: BlockType,
    pub position: Vec2,
    pub size: f32,
}

impl Block {
    pub fn new(position: Vec2, size: f32, block_type: BlockType) -> Self {
        Self {
            position,
            size,
            block_type,
        }
    }
}
