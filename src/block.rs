use macroquad::{
    color::{Color, colors},
    shapes::draw_rectangle,
};
use ordered_float::OrderedFloat;
use strum::EnumIter;

use crate::has_bounds::{Bounds, HasBounds};

pub enum BlockState {
    Default,
    Hover,
}

#[derive(EnumIter, Clone, Eq, PartialEq, Hash)]
pub enum BlockType {
    Potion,
    Blood,
    Bone,
    Poison,
    Coffin,
    Amber,
}

impl BlockType {
    fn get_color(&self) -> Color {
        match self {
            BlockType::Potion => colors::GREEN,
            BlockType::Blood => colors::RED,
            BlockType::Bone => colors::BEIGE,
            BlockType::Poison => colors::PURPLE,
            BlockType::Coffin => colors::BLACK,
            BlockType::Amber => colors::ORANGE,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct Block {
    x: OrderedFloat<f32>,
    y: OrderedFloat<f32>,
    size: OrderedFloat<f32>,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(x: f32, y: f32, size: f32, block_type: BlockType) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            size: OrderedFloat(size),
            block_type,
        }
    }

    pub fn x(&self) -> f32 {
        self.x.into_inner()
    }

    pub fn y(&self) -> f32 {
        self.y.into_inner()
    }

    pub fn set_y(&mut self, y: f32) {
        self.y.0 = y;
    }

    pub fn draw(&self, state: BlockState) {
        let color = match state {
            BlockState::Default => self.block_type.get_color(),
            BlockState::Hover => colors::LIGHTGRAY,
        };

        draw_rectangle(
            self.x(),
            self.y(),
            self.size.into_inner(),
            self.size.into_inner(),
            color,
        );
    }
}

impl HasBounds for Block {
    fn get_bounds(&self) -> Bounds {
        Bounds {
            left: self.x(),
            right: self.x() + self.size.into_inner(),
            top: self.y(),
            bottom: self.y() + self.size.into_inner(),
        }
    }
}
