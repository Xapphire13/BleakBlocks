use std::collections::HashSet;

use macroquad::{
    math::{Vec2, vec2},
    rand,
};
use strum::IntoEnumIterator;

use crate::{
    block::{Block, BlockState, BlockType},
    coordinate::{Coordinate, coordinate},
    physics_system::apply_force,
};

/// Force in pixels per second^2 that is applied to moving blocks
const FORCE: f32 = 2000.0;

pub struct GridLayout {
    rows: u32,
    cols: u32,
    dimensions: Vec2,
    position: Vec2,
    block_size: f32,
    /// Rows then Columns (top to bottom)
    blocks: Vec<Vec<Option<Block>>>,
    blocks_remaining: u32,
}

impl GridLayout {
    pub fn new(position: Vec2, dimensions: Vec2, rows: u32, cols: u32) -> Self {
        let block_size = (dimensions.x / cols as f32).min(dimensions.y / rows as f32);
        let block_types = BlockType::iter().collect::<Vec<_>>();

        let mut block_rows = vec![];
        for row in 0..rows {
            let mut block_row = vec![];

            for col in 0..cols {
                let local_position = Vec2::new(col as f32 * block_size, row as f32 * block_size);
                let world_position = position + local_position;
                let block_type = block_types[rand::rand() as usize % block_types.len()].clone();
                block_row.push(Some(Block::new(world_position, block_size, block_type)));
            }
            block_rows.push(block_row);
        }

        GridLayout {
            position,
            dimensions,
            rows,
            cols,
            block_size,
            blocks: block_rows,
            blocks_remaining: cols * rows,
        }
    }

    pub fn grid_to_world(&self, position: Coordinate) -> Vec2 {
        Vec2::new(
            self.position.x + position.col as f32 * self.block_size,
            self.position.y + position.row as f32 * self.block_size,
        )
    }

    pub fn world_to_grid(&self, world_pos: Vec2) -> Option<Coordinate> {
        if !self.contains_point(world_pos) {
            return None;
        }

        let local_pos = world_pos - self.position;
        let row = (local_pos.y / self.block_size) as u32;
        let col = (local_pos.x / self.block_size) as u32;

        Some(coordinate(row, col))
    }

    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.dimensions.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.dimensions.y
    }

    pub fn draw(&self, hovered_blocks: HashSet<Coordinate>) {
        // Render blocks
        for row in 0..self.rows {
            for col in 0..self.cols {
                let position = coordinate(row, col);
                if let Some(block) = self.get_block_at_grid_position(position) {
                    let block_state = if hovered_blocks.contains(&position) {
                        BlockState::Hover
                    } else {
                        BlockState::Default
                    };
                    block.draw(block_state);
                }
            }
        }
    }

    fn get_block_at_grid_position(&self, position: Coordinate) -> Option<&Block> {
        self.blocks
            .get(position.row as usize)?
            .get(position.col as usize)?
            .as_ref()
    }

    pub fn get_block_region(&self, start: Coordinate) -> HashSet<Coordinate> {
        let mut region = HashSet::new();
        let block_type = 'block_type: {
            if let Some(block) = self.get_block_at_grid_position(start) {
                break 'block_type block.block_type.clone();
            } else {
                return region;
            }
        };

        let mut neighbors = vec![start];
        while let Some(position) = neighbors.pop() {
            if let Some(block) = self.get_block_at_grid_position(position) {
                if !region.contains(&position) && block.block_type == block_type {
                    neighbors.extend(position.get_neighbors(self.rows, self.cols));
                    region.insert(position);
                }
            }
        }

        region
    }

    pub fn remove_block_region(&mut self, start_position: Vec2) {
        let start_coordinate = self.world_to_grid(start_position);

        if start_coordinate.is_none() {
            return;
        }
        let start_coordinate = start_coordinate.unwrap();

        let block_positions = self
            .get_block_region(start_coordinate)
            .into_iter()
            .collect::<Vec<_>>();

        for &Coordinate { row, col } in block_positions.iter() {
            self.blocks[row as usize][col as usize].take();
        }

        self.blocks_remaining -= block_positions.len() as u32;
    }

    /// Returns true if there are gaps in the grid (i.e. blocks have been removed and need to be rearranged)
    pub fn has_gaps(&self) -> bool {
        for col in 0..self.cols {
            let mut found_gap = false;

            for row in (0..self.rows).rev() {
                if self.blocks[row as usize][col as usize].is_none() {
                    found_gap = true;
                } else if found_gap {
                    // We encountered a block after a gap (the block needs to fall)
                    return true;
                }
            }
        }

        false
    }

    fn is_column_empty(&self, col: u32) -> bool {
        for row in (0..self.rows).rev() {
            if self.blocks[row as usize][col as usize].is_some() {
                return false;
            }
        }

        true
    }

    /// Returns true if any columns need shifting due to empty columns in the grid
    pub fn columns_need_shifting(&self) -> bool {
        let mut found_empty_column = false;
        for col in 0..self.cols {
            if self.is_column_empty(col) {
                found_empty_column = true;
            } else if found_empty_column {
                // We encountered a non-empty column after an empty column (columns need to shift)
                return true;
            }
        }

        false
    }

    pub fn animate_falling(&mut self, time_delta: f32) {
        for col in 0..self.cols {
            let mut empty_spaces = 0;
            for row in (0..self.rows).rev() {
                let original_grid_position = coordinate(row, col);
                if self.blocks[original_grid_position.row as usize]
                    [original_grid_position.col as usize]
                    .is_none()
                {
                    empty_spaces += 1;
                } else if empty_spaces > 0 {
                    if let Some(mut block) = self.blocks[original_grid_position.row as usize]
                        [original_grid_position.col as usize]
                        .take()
                    {
                        let terminal_grid_position =
                            original_grid_position + coordinate(empty_spaces, 0);
                        let terminal_world_position = self.grid_to_world(terminal_grid_position);
                        apply_force(&mut block, vec2(0.0, FORCE), time_delta);

                        if block.position.y >= terminal_world_position.y {
                            block.position = terminal_world_position;
                            block.velocity = Vec2::ZERO;
                            self.blocks[terminal_grid_position.row as usize]
                                [terminal_grid_position.col as usize]
                                .replace(block);
                        } else {
                            // Put the block back, its not in its final location yet
                            self.blocks[original_grid_position.row as usize]
                                [original_grid_position.col as usize]
                                .replace(block);
                        }
                    }
                }
            }
        }
    }

    pub fn shift_columns(&mut self, time_delta: f32) {
        let mut empty_columns = 0;

        for col in 0..self.cols {
            if self.is_column_empty(col) {
                empty_columns += 1;
                continue;
            }

            for row in 0..self.rows {
                let original_grid_position = coordinate(row, col);
                let terminal_grid_position = original_grid_position - coordinate(0, empty_columns);
                let terminal_world_position = self.grid_to_world(terminal_grid_position);

                if let Some(mut block) = self.blocks[original_grid_position.row as usize]
                    [original_grid_position.col as usize]
                    .take()
                {
                    apply_force(&mut block, vec2(-FORCE, 0.0), time_delta);

                    if block.position.x <= terminal_world_position.x {
                        block.position = terminal_world_position;
                        block.velocity = Vec2::ZERO;
                        self.blocks[terminal_grid_position.row as usize]
                            [terminal_grid_position.col as usize]
                            .replace(block);
                    } else {
                        // Put the block back, its not in its final location yet
                        self.blocks[original_grid_position.row as usize]
                            [original_grid_position.col as usize]
                            .replace(block);
                    }
                }
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.blocks_remaining == 0
    }
}
