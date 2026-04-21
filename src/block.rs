use macroquad::color::Color;
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
    pub fn get_color(&self) -> Color {
        match self {
            BlockType::Potion => Color::from_hex(0x40FF00),
            BlockType::Blood => Color::from_hex(0xE01F39),
            BlockType::Ghost => Color::from_hex(0xFFFFFF),
            BlockType::Poison => Color::from_hex(0x9001FE),
            BlockType::Coffin => Color::from_hex(0xAA7855),
            BlockType::Gravestone => Color::from_hex(0x788087),
            BlockType::Flame => Color::from_hex(0xFFA118),
            BlockType::Brain => Color::from_hex(0xFF00F2),
        }
    }

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
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}
