use std::collections::HashSet;

use macroquad::rand;
use strum::IntoEnumIterator;

use crate::{
    block::{Block, BlockState, BlockType},
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
    blocks: Vec<Vec<Option<Block>>>,
    blocks_remaining: u32,
}

impl GameGrid {
    pub fn new(x: f32, y: f32, width: f32, height: f32, rows: u32, cols: u32) -> Self {
        let block_size = (width / cols as f32).min(height / rows as f32);
        let block_types = BlockType::iter().collect::<Vec<_>>();

        let mut block_rows = vec![];
        for row in 0..rows {
            let mut block_row = vec![];

            for col in 0..cols {
                let x = GameGrid::col_to_x(x, block_size, col);
                let y = GameGrid::row_to_y(y, block_size, row);
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
            blocks_remaining: cols * rows,
        }
    }

    fn row_to_y(base_y: f32, block_size: f32, row: u32) -> f32 {
        base_y + row as f32 * block_size
    }

    fn col_to_x(base_x: f32, block_size: f32, col: u32) -> f32 {
        base_x + col as f32 * block_size
    }

    pub fn draw(&self, hovered_blocks: HashSet<&Block>) {
        // Render blocks
        for block in self.blocks.iter().flatten() {
            if let Some(block) = block.as_ref() {
                let block_state = if hovered_blocks.contains(block) {
                    BlockState::Hover
                } else {
                    BlockState::Default
                };
                block.draw(block_state);
            }
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

        return true;
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

    pub fn animate_falling(&mut self, elapsed_time_seconds: f64) {
        for col in 0..self.cols {
            let mut empty_spaces = 0;
            for row in (0..self.rows).rev() {
                if self.blocks[row as usize][col as usize].is_none() {
                    empty_spaces += 1;
                } else if empty_spaces > 0 {
                    if let Some(mut block) = self.blocks[row as usize][col as usize].take() {
                        let terminal_row = row + empty_spaces;
                        let terminal_row_y =
                            GameGrid::row_to_y(self.y, self.block_size, terminal_row);
                        block.apply_gravity(elapsed_time_seconds);

                        if block.y() >= terminal_row_y {
                            block.set_y(terminal_row_y);
                            block.set_velocity(0.0);
                            self.blocks[terminal_row as usize][col as usize].replace(block);
                        } else {
                            // Put the block back, its not in its final location yet
                            self.blocks[row as usize][col as usize].replace(block);
                        }
                    }
                }
            }
        }
    }

    pub fn shift_columns(&mut self, elapsed_time_seconds: f64) {
        let mut empty_columns = 0;

        for col in 0..self.cols {
            if self.is_column_empty(col) {
                empty_columns += 1;
                continue;
            }

            let terminal_col = col - empty_columns;
            let terminal_col_x = GameGrid::col_to_x(self.x, self.block_size, terminal_col);

            for row in 0..self.rows {
                if let Some(mut block) = self.blocks[row as usize][col as usize].take() {
                    block.apply_gravity_left(elapsed_time_seconds);

                    if block.x() <= terminal_col_x {
                        block.set_x(terminal_col_x);
                        block.set_velocity(0.0);
                        self.blocks[row as usize][terminal_col as usize].replace(block);
                    } else {
                        // Put the block back, its not in its final location yet
                        self.blocks[row as usize][col as usize].replace(block);
                    }
                }
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.blocks_remaining == 0
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
