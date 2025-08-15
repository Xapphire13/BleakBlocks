use std::collections::HashSet;

use macroquad::{
    color::colors,
    prelude::*,
    rand::{self},
};
use ordered_float::OrderedFloat;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    fps_limiter::FpsLimiter,
    has_bounds::{Bounds, HasBounds},
};

mod fps_limiter;
mod has_bounds;

const BACKGROUND_COLOR: u32 = 0x31263E;
const GRID_MARGIN: f32 = 20.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Bleak Blocks".to_owned(),
        window_height: 500,
        window_width: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fps_limiter = FpsLimiter::new(60.0);
    let grid_size = screen_width().min(screen_height()) - 2. * GRID_MARGIN;
    let mut grid = GameGrid::new(GRID_MARGIN, GRID_MARGIN, grid_size, grid_size, 10, 10);

    loop {
        clear_background(Color::from_hex(BACKGROUND_COLOR));

        // Update hover state
        let (mouse_x, mouse_y) = mouse_position();

        // Remove blocks when clicked
        if is_mouse_button_down(MouseButton::Left) {
            grid.remove_block_region(mouse_x, mouse_y);
        }

        let hovered_blocks = if let Some(block) = grid.get_block_at_pixel_position(mouse_x, mouse_y)
        {
            grid.get_block_region(block)
        } else {
            HashSet::new()
        };

        // Draw game
        for block in grid.blocks.iter().flatten() {
            if let Some(block) = block.as_ref() {
                let block_state = if hovered_blocks.contains(block) {
                    BlockState::Hover
                } else {
                    BlockState::Default
                };
                block.draw(block_state);
            }
        }
        grid.draw();

        fps_limiter.wait_for_next_frame();
        next_frame().await
    }
}

enum BlockState {
    Default,
    Hover,
}

#[derive(EnumIter, Clone, Eq, PartialEq, Hash)]
enum BlockType {
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
struct Block {
    x: OrderedFloat<f32>,
    y: OrderedFloat<f32>,
    size: OrderedFloat<f32>,
    block_type: BlockType,
}

impl Block {
    fn new(x: f32, y: f32, size: f32, block_type: BlockType) -> Self {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            size: OrderedFloat(size),
            block_type,
        }
    }

    fn draw(&self, state: BlockState) {
        let color = match state {
            BlockState::Default => self.block_type.get_color(),
            BlockState::Hover => colors::LIGHTGRAY,
        };

        draw_rectangle(
            self.x.into_inner(),
            self.y.into_inner(),
            self.size.into_inner(),
            self.size.into_inner(),
            color,
        );
    }
}

impl HasBounds for Block {
    fn get_bounds(&self) -> Bounds {
        Bounds {
            left: self.x.into_inner(),
            right: self.x.into_inner() + self.size.into_inner(),
            top: self.y.into_inner(),
            bottom: self.y.into_inner() + self.size.into_inner(),
        }
    }
}

struct GameGrid {
    rows: u32,
    cols: u32,
    width: f32,
    height: f32,
    block_size: f32,
    x: f32,
    y: f32,
    /// Rows then Columns (top to bottom)
    blocks: Vec<Vec<Option<Block>>>,
}

impl GameGrid {
    fn new(x: f32, y: f32, width: f32, height: f32, rows: u32, cols: u32) -> Self {
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

    fn draw(&self) {
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

    fn get_block_at_pixel_position(&self, x: f32, y: f32) -> Option<&Block> {
        let (row, col) = self.get_grid_position(x, y)?;

        self.get_block_at_grid_position(row, col)
    }

    fn get_block_at_grid_position(&self, row: u32, col: u32) -> Option<&Block> {
        if row < self.rows && col < self.cols {
            return self.blocks[row as usize][col as usize].as_ref();
        }

        None
    }

    fn get_block_region<'a>(&'a self, start_block: &'a Block) -> HashSet<&'a Block> {
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
        let grid_position = self.get_grid_position(block.x.into_inner(), block.y.into_inner());

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

    fn remove_block_region(&mut self, x: f32, y: f32) {
        let start_block = self.get_block_at_pixel_position(x, y);

        if start_block.is_none() {
            return;
        }

        let start_block = start_block.unwrap();
        let block_positions = self
            .get_block_region(start_block)
            .iter()
            .map(|block| self.get_grid_position(block.x.into_inner(), block.y.into_inner()))
            .flatten()
            .collect::<Vec<_>>();

        for &(row, col) in block_positions.iter() {
            self.blocks[row as usize][col as usize] = None;
        }
    }
}

impl HasBounds for GameGrid {
    fn get_bounds(&self) -> has_bounds::Bounds {
        Bounds {
            left: self.x,
            right: self.x + self.width,
            top: self.y,
            bottom: self.y + self.height,
        }
    }
}
