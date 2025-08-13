use macroquad::{
    prelude::*,
    rand::{self},
};

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
    let grid = GameGrid::new(GRID_MARGIN, GRID_MARGIN, grid_size, 10, 10);

    loop {
        clear_background(WHITE);

        grid.draw();

        next_frame().await
    }
}

struct Block {
    row: i32,
    col: i32,
    color: Color,
}

impl Block {
    fn new(row: i32, col: i32, color: Color) -> Self {
        Self { row, col, color }
    }

    fn draw(&self, parent_grid: &GameGrid) {
        draw_rectangle(
            parent_grid.x + self.col as f32 * parent_grid.block_size(),
            parent_grid.y + self.row as f32 * parent_grid.block_size(),
            parent_grid.block_size(),
            parent_grid.block_size(),
            self.color,
        );
    }
}

struct GameGrid {
    rows: i32,
    cols: i32,
    size: f32,
    x: f32,
    y: f32,
    blocks: Vec<Block>,
}

impl GameGrid {
    fn new(x: f32, y: f32, size: f32, rows: i32, cols: i32) -> Self {
        let mut blocks = vec![];

        for row in 0..rows {
            for col in 0..cols {
                let color = COLORS[rand::rand() as usize % COLORS.len()];
                blocks.push(Block::new(row, col, color));
            }
        }

        GameGrid {
            x,
            y,
            rows,
            cols,
            size,
            blocks,
        }
    }

    fn block_size(&self) -> f32 {
        // Blocks are square
        self.size / self.cols as f32
    }

    fn draw(&self) {
        for block in self.blocks.iter() {
            block.draw(self);
        }

        draw_rectangle_lines(self.x, self.y, self.size, self.size, 2., BLACK);

        // Draw rows
        for i in 1..self.rows {
            let y = i as f32 * self.block_size();
            draw_line(
                self.x,
                self.y + y,
                self.x + self.size,
                self.y + y,
                1.,
                BLACK,
            );
        }

        // Draw columns
        for i in 1..self.cols {
            let x = i as f32 * self.block_size();
            draw_line(
                self.x + x,
                self.y,
                self.x + x,
                self.y + self.size,
                1.,
                BLACK,
            );
        }
    }
}
