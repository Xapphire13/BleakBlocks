use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{coordinate::Coordinate, grid_layout::GridLayout};

pub struct PhysicsSystem {
    animating_blocks: HashMap<Coordinate, AnimationState>,
    velocity: Vec2,
}

struct AnimationState {
    target: Coordinate,
    offset: Vec2, // Current offset from grid position
    completed: bool,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            animating_blocks: HashMap::new(),
            velocity: Vec2::ZERO,
        }
    }

    pub fn queue_block_animation(&mut self, from: Coordinate, to: Coordinate) {
        self.animating_blocks.insert(
            from,
            AnimationState {
                target: to,
                offset: Vec2::ZERO,
                completed: false,
            },
        );
    }

    /// Updates all animating blocks
    /// Returns true if animations are still in progress
    pub fn update(&mut self, layout: &mut GridLayout, force: Vec2, time_delta: f32) -> bool {
        if self.animating_blocks.is_empty() {
            return false;
        }

        self.velocity += force * time_delta;

        self.update_block_positions(layout, time_delta);
        self.finalize_completed_animations(layout);

        !self.animating_blocks.is_empty()
    }

    fn update_block_positions(&mut self, layout: &mut GridLayout, time_delta: f32) {
        let displacement = self.velocity * time_delta;

        for (from, animation_state) in &mut self.animating_blocks {
            animation_state.offset += displacement;

            let start_pos = layout.grid_to_world(*from);
            let target_pos = layout.grid_to_world(animation_state.target);
            let total_distance = target_pos - start_pos;

            if animation_state.offset.length() >= total_distance.length() {
                animation_state.offset = total_distance;
                animation_state.completed = true;
            }
        }
    }

    fn finalize_completed_animations(&mut self, layout: &mut GridLayout) {
        if self
            .animating_blocks
            .iter()
            .all(|(_, state)| state.completed)
        {
            // All animations complete - reset velocity and finalize positions
            self.velocity = Vec2::ZERO;

            // Remove blocks from original grid positions, then put them all into their new grid positions
            for (block, to) in self
                .animating_blocks
                .drain()
                .flat_map(|(current_pos, animation_state)| {
                    layout
                        .take_block(current_pos)
                        .map(|block| (block, animation_state.target))
                })
                .collect::<Vec<_>>()
            {
                layout.place_block(to, block);
            }
        }
    }

    pub fn get_animation_offset(&self, coord: Coordinate) -> Vec2 {
        self.animating_blocks
            .get(&coord)
            .map(|anim| anim.offset)
            .unwrap_or(Vec2::ZERO)
    }
}
