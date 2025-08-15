use std::collections::HashSet;

use macroquad::{color::Color, rand, shapes::draw_line};
use strum::IntoEnumIterator;

use crate::{
    BACKGROUND_COLOR, BLOCK_FALL_SPEED,
    block::{Block, BlockType},
    has_bounds::{Bounds, HasBounds},
};

pub struct GameGrid {
    rows: u32,
    cols: u32,
    width: f32,
    height: f32,
    block_size: f32,
    x: f32,
    y: f32,
    /// Rows then Columns (top to bottom)
    pub blocks: Vec<Vec<Option<Block>>>,
}

impl GameGrid {
    pub fn new(x: f32, y: f32, width: f32, height: f32, rows: u32, cols: u32) -> Self {
        let block_size = (width / cols as f32).min(height / rows as f32);
        let block_types = BlockType::iter().collect::<Vec<_>>();

        let mut block_rows = vec![];
        for row in 0..rows {
            let mut block_row = vec![];

            for col in 0..cols {
                let x = x + col as f32 * block_size;
                let y = y + row as f32 * block_size;
                let block_type = block_types[rand::rand() as usize % block_types.len()].clone();
                block_row.push(Some(Block::new(x, y, block_size, block_type)));
            }
            block_rows.push(block_row);
        }

        GameGrid {
            x,
            y,
            rows,
            cols,
            width,
            height,
            block_size,
            blocks: block_rows,
        }
    }

    pub fn draw(&self) {
        let color = Color::from_hex(BACKGROUND_COLOR);

        // Draw rows
        for i in 1..self.rows {
            let y = i as f32 * self.block_size;
            draw_line(
                self.x,
                self.y + y,
                self.x + self.width,
                self.y + y,
                1.,
                color,
            );
        }

        // Draw columns
        for i in 1..self.cols {
            let x = i as f32 * self.block_size;
            draw_line(
                self.x + x,
                self.y,
                self.x + x,
                self.y + self.height,
                1.,
                color,
            );
        }
    }

    fn get_grid_position(&self, x: f32, y: f32) -> Option<(u32, u32)> {
        if !self.is_within_bounds(x, y) {
            return None;
        }

        let row = (y - self.y) as u32 / self.block_size as u32;
        let col = (x - self.x) as u32 / self.block_size as u32;

        Some((row, col))
    }

    pub fn get_block_at_pixel_position(&self, x: f32, y: f32) -> Option<&Block> {
        let (row, col) = self.get_grid_position(x, y)?;

        self.get_block_at_grid_position(row, col)
    }

    fn get_block_at_grid_position(&self, row: u32, col: u32) -> Option<&Block> {
        if row < self.rows && col < self.cols {
            return self.blocks[row as usize][col as usize].as_ref();
        }

        None
    }

    pub fn get_block_region<'a>(&'a self, start_block: &'a Block) -> HashSet<&'a Block> {
        let mut region = HashSet::new();

        let mut neighbors = vec![start_block];

        while let Some(block) = neighbors.pop() {
            region.insert(block);

            for neighbor in self.get_neighbors(block) {
                if !region.contains(neighbor) && neighbor.block_type == block.block_type {
                    neighbors.push(neighbor);
                }
            }
        }

        region
    }

    fn get_neighbors(&self, block: &Block) -> Vec<&Block> {
        let mut neighbors = Vec::new();
        let grid_position = self.get_grid_position(block.x(), block.y());

        if grid_position.is_none() {
            return vec![];
        }
        let (row, col) = grid_position.unwrap();

        // Up
        if row > 0 {
            if let Some(block) = self.get_block_at_grid_position(row - 1, col) {
                neighbors.push(block);
            }
        }

        // Down
        if row < self.rows - 1 {
            if let Some(block) = self.get_block_at_grid_position(row + 1, col) {
                neighbors.push(block);
            }
        }

        // Left
        if col > 0 {
            if let Some(block) = self.get_block_at_grid_position(row, col - 1) {
                neighbors.push(block);
            }
        }

        // Right
        if col < self.cols - 1 {
            if let Some(block) = self.get_block_at_grid_position(row, col + 1) {
                neighbors.push(block);
            }
        }

        neighbors
    }

    pub fn remove_block_region(&mut self, x: f32, y: f32) {
        let start_block = self.get_block_at_pixel_position(x, y);

        if start_block.is_none() {
            return;
        }

        let start_block = start_block.unwrap();
        let block_positions = self
            .get_block_region(start_block)
            .iter()
            .filter_map(|block| self.get_grid_position(block.x(), block.y()))
            .collect::<Vec<_>>();

        for &(row, col) in block_positions.iter() {
            self.blocks[row as usize][col as usize] = None;
        }
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

    pub fn animate_falling(&mut self, time_delta_seconds: f64) {
        for col in 0..self.cols {
            for row in (0..self.rows).rev() {
                if (row + 1 < self.rows) && self.blocks[row as usize + 1][col as usize].is_none() {
                    let mut new_position = None;
                    if let Some(block) = self.blocks[row as usize][col as usize].as_mut() {
                        block.set_y(
                            block.y() + (BLOCK_FALL_SPEED as f64 * time_delta_seconds) as f32,
                        );
                        new_position = Some((block.x(), block.y()));
                    }

                    if let Some((new_x, new_y)) = new_position {
                        if let Some((new_row, ..)) = self.get_grid_position(new_x, new_y) {
                            if new_row != row {
                                // block moved into new grid position
                                let mut block = self.blocks[row as usize][col as usize].take();

                                // Ensure it stops on a clean block boundary
                                if let Some(block) = block.as_mut() {
                                    block.set_y(self.y + new_row as f32 * self.block_size);
                                }

                                self.blocks[new_row as usize][col as usize] = block;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl HasBounds for GameGrid {
    fn get_bounds(&self) -> Bounds {
        Bounds {
            left: self.x,
            right: self.x + self.width,
            top: self.y,
            bottom: self.y + self.height,
        }
    }
}
