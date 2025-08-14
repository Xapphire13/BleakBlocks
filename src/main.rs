use std::cell::RefCell;

use macroquad::{
    color::colors,
    prelude::*,
    rand::{self},
};

use crate::has_bounds::{Bounds, HasBounds};

mod has_bounds;

const GRID_MARGIN: f32 = 20.0;
const COLORS: [Color; 6] = [GREEN, BLUE, PURPLE, RED, YELLOW, ORANGE];

fn window_conf() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").to_string(),
        window_height: 500,
        window_width: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let grid_size = screen_width().min(screen_height()) - 2. * GRID_MARGIN;
    let grid = GameGrid::new(GRID_MARGIN, GRID_MARGIN, grid_size, grid_size, 10, 10);

    loop {
        clear_background(WHITE);

        // Update hover state
        let (mouse_x, mouse_y) = mouse_position();
        let hovered_block = grid.get_block_at(mouse_x, mouse_y);
        for block in grid.blocks.iter() {
            if hovered_block.is_some_and(|it| std::ptr::eq(it, block)) {
                *block.state.borrow_mut() = BlockState::Hover;
            } else {
                *block.state.borrow_mut() = BlockState::Default;
            }
        }

        // Draw game
        grid.draw();

        next_frame().await
    }
}

enum BlockState {
    Default,
    Hover,
}

struct Block {
    x: f32,
    y: f32,
    size: f32,
    row: i32,
    col: i32,
    color: Color,
    state: RefCell<BlockState>,
}

impl Block {
    fn new(x: f32, y: f32, size: f32, row: i32, col: i32, color: Color) -> Self {
        Self {
            x,
            y,
            size,
            row,
            col,
            color,
            state: RefCell::new(BlockState::Default),
        }
    }

    fn draw(&self) {
        let color = match *self.state.borrow() {
            BlockState::Default => self.color,
            BlockState::Hover => colors::LIGHTGRAY,
        };

        draw_rectangle(self.x, self.y, self.size, self.size, color);
    }
}

impl HasBounds for Block {
    fn get_bounds(&self) -> Bounds {
        Bounds {
            left: self.x,
            right: self.x + self.size,
            top: self.y,
            bottom: self.y + self.size,
        }
    }
}

struct GameGrid {
    rows: i32,
    cols: i32,
    width: f32,
    height: f32,
    block_size: f32,
    x: f32,
    y: f32,
    blocks: Vec<Block>,
}

impl GameGrid {
    fn new(x: f32, y: f32, width: f32, height: f32, rows: i32, cols: i32) -> Self {
        let mut blocks = vec![];
        let block_size = (width / cols as f32).min(height / rows as f32);

        for row in 0..rows {
            for col in 0..cols {
                let x = x + col as f32 * block_size;
                let y = y + row as f32 * block_size;
                let color = COLORS[rand::rand() as usize % COLORS.len()];
                blocks.push(Block::new(x, y, block_size, row, col, color));
            }
        }

        GameGrid {
            x,
            y,
            rows,
            cols,
            width,
            height,
            block_size,
            blocks,
        }
    }

    fn draw(&self) {
        for block in self.blocks.iter() {
            block.draw();
        }

        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2., BLACK);

        // Draw rows
        for i in 1..self.rows {
            let y = i as f32 * self.block_size;
            draw_line(
                self.x,
                self.y + y,
                self.x + self.width,
                self.y + y,
                1.,
                BLACK,
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
                BLACK,
            );
        }
    }

    fn get_block_at(&self, x: f32, y: f32) -> Option<&Block> {
        if self.is_within_bounds(x, y) {
            return self
                .blocks
                .iter()
                .find(|block| block.is_within_bounds(x, y));
        }

        None
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
