use std::collections::HashSet;

use macroquad::{
    math::{Rect, Vec2},
    rand,
};
use strum::IntoEnumIterator;

use crate::{
    block::{Block, BlockType},
    coordinate::{Coordinate, coordinate},
};

pub struct GridLayout {
    pub rows: u32,
    pub cols: u32,
    pub blocks_remaining: u32,
    pub block_size: f32,
    rect: Rect,
    /// Ordered row by row, top to bottom
    blocks: Vec<Option<Block>>,
}

impl GridLayout {
    pub fn new(position: Vec2, dimensions: Vec2, rows: u32, cols: u32) -> Self {
        let block_size = (dimensions.x / cols as f32).min(dimensions.y / rows as f32);
        let block_types = BlockType::iter().collect::<Vec<_>>();

        let mut blocks = vec![];
        for _ in 0..(rows * cols) {
            let block_type = block_types[rand::rand() as usize % block_types.len()].clone();
            blocks.push(Some(Block::new(block_size, block_type)));
        }

        GridLayout {
            rect: Rect {
                x: position.x,
                y: position.y,
                w: dimensions.x,
                h: dimensions.y,
            },
            rows,
            cols,
            block_size,
            blocks,
            blocks_remaining: cols * rows,
        }
    }

    pub fn grid_to_world(&self, position: Coordinate) -> Vec2 {
        Vec2::new(
            self.x() + position.col as f32 * self.block_size,
            self.y() + position.row as f32 * self.block_size,
        )
    }

    pub fn x(&self) -> f32 {
        self.rect.x
    }

    pub fn y(&self) -> f32 {
        self.rect.y
    }

    pub fn width(&self) -> f32 {
        self.rect.w
    }

    pub fn height(&self) -> f32 {
        self.rect.h
    }

    pub fn world_to_grid(&self, world_pos: Vec2) -> Option<Coordinate> {
        if !self.rect.contains(world_pos) {
            return None;
        }

        let local_pos = world_pos - self.rect.point();
        let row = (local_pos.y / self.block_size) as u32;
        let col = (local_pos.x / self.block_size) as u32;

        Some(coordinate(row, col))
    }

    fn get_index(&self, position: Coordinate) -> usize {
        (position.row * self.rows + position.col) as usize
    }

    pub fn get_block(&self, position: Coordinate) -> Option<&Block> {
        let index = self.get_index(position);
        self.blocks.get(index)?.as_ref()
    }

    pub fn take_block(&mut self, position: Coordinate) -> Option<Block> {
        let index = self.get_index(position);

        if index >= self.blocks.len() {
            return None;
        }

        self.blocks[index].take()
    }

    pub fn get_block_region(&self, start: Coordinate) -> HashSet<Coordinate> {
        let mut region = HashSet::new();
        let block_type = 'block_type: {
            if let Some(block) = self.get_block(start) {
                break 'block_type block.block_type.clone();
            } else {
                return region;
            }
        };

        let mut neighbors = vec![start];
        while let Some(position) = neighbors.pop() {
            if let Some(block) = self.get_block(position) {
                if !region.contains(&position) && block.block_type == block_type {
                    neighbors.extend(position.get_neighbors(self.rows, self.cols));
                    region.insert(position);
                }
            }
        }

        region
    }

    // Returns number of blocks removed
    pub fn remove_block_region(&mut self, start_position: Vec2) -> u32 {
        let start_coordinate = self.world_to_grid(start_position);

        if start_coordinate.is_none() {
            return 0;
        }
        let start_coordinate = start_coordinate.unwrap();

        let block_positions = self
            .get_block_region(start_coordinate)
            .into_iter()
            .collect::<Vec<_>>();

        for &position in block_positions.iter() {
            self.take_block(position);
        }

        self.blocks_remaining -= block_positions.len() as u32;
        block_positions.len() as u32
    }

    pub fn find_falling_blocks(&self) -> Option<Vec<(Coordinate, Coordinate)>> {
        let mut result = vec![];

        for col in 0..self.cols {
            let mut rows_to_fall = 0u32;

            for row in (0..self.rows).rev() {
                let position = coordinate(row, col);
                if self.is_empty_at(position) {
                    rows_to_fall += 1;
                } else if rows_to_fall > 0 {
                    // We encountered a block after a gap (the block needs to fall)
                    result.push((position, coordinate(row + rows_to_fall, col)));
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn find_shifting_blocks(&self) -> Option<Vec<(Coordinate, Coordinate)>> {
        let mut result = vec![];

        let mut columns_to_shift = 0;
        for col in 0..self.cols {
            if self.is_column_empty(col) {
                columns_to_shift += 1;
            } else if columns_to_shift > 0 {
                // We encountered a non-empty column after an empty column (columns need to shift)
                for row in 0..self.rows {
                    let position = coordinate(row, col);
                    if !self.is_empty_at(position) {
                        result.push((position, coordinate(row, col - columns_to_shift)));
                    }
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn is_column_empty(&self, col: u32) -> bool {
        for row in (0..self.rows).rev() {
            if self.get_block(coordinate(row, col)).is_some() {
                return false;
            }
        }

        true
    }

    pub fn place_block(&mut self, position: Coordinate, block: Block) {
        let index = self.get_index(position);

        if index < self.blocks.len() {
            self.blocks[index].replace(block);
        }
    }

    pub fn is_empty_at(&self, position: Coordinate) -> bool {
        self.get_block(position).is_none()
    }
}
