use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{block::Block, coordinate::Coordinate, grid_layout::GridLayout};

pub struct PhysicsSystem {
    animating_blocks: HashMap<Coordinate, Coordinate>,
    completed_blocks: Vec<(Coordinate, Coordinate)>,
    velocity: Vec2,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            animating_blocks: HashMap::new(),
            completed_blocks: Vec::new(),
            velocity: Vec2::ZERO,
        }
    }

    pub fn queue_block_animation(&mut self, from: Coordinate, to: Coordinate) {
        self.animating_blocks.insert(from, to);
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
        let blocks_to_update: Vec<_> = self
            .animating_blocks
            .iter()
            .map(|(from, to)| (*from, *to))
            .collect();

        for (from, to) in blocks_to_update {
            if let Some(mut block) = layout.take_block(from) {
                block.position += self.velocity * time_delta;

                if self.is_animation_complete(&block, layout, from, to) {
                    self.complete_block_animation(from, to, block, layout);
                } else {
                    // Put block back, we're not done with it
                    layout.place_block(from, block);
                }
            }
        }
    }

    /// Check if a block has reached its destination
    fn is_animation_complete(
        &self,
        block: &Block,
        layout: &GridLayout,
        from: Coordinate,
        to: Coordinate,
    ) -> bool {
        let start_pos = layout.grid_to_world(from);
        let end_pos = layout.grid_to_world(to);
        let total_distance = end_pos.distance(start_pos);
        let current_distance = start_pos.distance(block.position);

        current_distance >= total_distance
    }

    /// Mark a block animation as complete
    fn complete_block_animation(
        &mut self,
        from: Coordinate,
        to: Coordinate,
        mut block: Block,
        layout: &mut GridLayout,
    ) {
        // Snap to exact final position
        block.position = layout.grid_to_world(to);

        // Remove from active animations and queue for final placement
        self.animating_blocks.remove(&from);
        self.completed_blocks.push((from, to));

        // Temporarily place at original position (will be moved in finalize step)
        layout.place_block(from, block);
    }

    fn finalize_completed_animations(&mut self, layout: &mut GridLayout) {
        if self.animating_blocks.is_empty() && !self.completed_blocks.is_empty() {
            // All animations complete - reset velocity and finalize positions
            self.velocity = Vec2::ZERO;

            for (block, to) in self
                .completed_blocks
                .drain(..)
                .flat_map(|(from, to)| layout.take_block(from).map(|block| (block, to)))
                .collect::<Vec<_>>()
            {
                layout.place_block(to, block);
            }
        }
    }
}
