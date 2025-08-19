use macroquad::math::Vec2;

use crate::block::Block;

pub fn apply_force(block: &mut Block, force: Vec2, time_delta: f32) {
    block.position += block.velocity * time_delta;
    block.velocity += force * time_delta;
}
