use std::collections::HashMap;

use macroquad::math::Vec2;

use crate::{coordinate::Coordinate, grid_layout::GridLayout};

pub struct PhysicsSystem {
    tracked_blocks: HashMap<Coordinate, Coordinate>,
    settled_blocks: HashMap<Coordinate, Coordinate>,
    velocity: Vec2,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            tracked_blocks: HashMap::new(),
            settled_blocks: HashMap::new(),
            velocity: Vec2::ZERO,
        }
    }

    pub fn track_block(&mut self, from: Coordinate, to: Coordinate) {
        self.tracked_blocks.insert(from, to);
    }

    /// Applies force to all tracked blocks
    /// Returns true if there are still blocks remaining to be moved
    pub fn update(&mut self, layout: &mut GridLayout, force: Vec2, time_delta: f32) -> bool {
        self.velocity += force * time_delta;

        let blocks_to_update: Vec<_> = self
            .tracked_blocks
            .iter()
            .map(|(from, to)| (*from, *to))
            .collect();
        for (from, to) in blocks_to_update {
            if let Some(mut block) = layout.take_block(from) {
                block.position += self.velocity * time_delta;

                let start_pos = layout.grid_to_world(from);
                let end_pos = layout.grid_to_world(to);
                let total_distance = end_pos.distance(start_pos);
                let current_distance = start_pos.distance(block.position);
                if current_distance >= total_distance {
                    // Travelled to (or further than) destination
                    block.position = end_pos; // Snap to end position to prevent over-shooting
                    // layout.place_block(to, block);
                    self.tracked_blocks.remove(&from);
                    self.settled_blocks.insert(from, to);
                }

                layout.place_block(from, block);
            }
        }

        if !self.tracked_blocks.is_empty() {
            true
        } else {
            self.velocity = Vec2::ZERO;
            self.settled_blocks
                .drain()
                .flat_map(|(from, to)| layout.take_block(from).map(|block| (block, to)))
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(block, to)| layout.place_block(to, block));

            false
        }
    }
}
