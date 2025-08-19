use macroquad::math::{Vec2, vec2};

use crate::{block::Block, constants::FORCE, coordinate::coordinate, grid_layout::GridLayout};

pub fn apply_force(block: &mut Block, force: Vec2, time_delta: f32) {
    block.position += block.velocity * time_delta;
    block.velocity += force * time_delta;
}

/// Returns true if blocks are still moving
pub fn animate_blocks_falling(grid_layout: &mut GridLayout, time_delta: f32) -> bool {
    let mut blocks_still_moving = false;
    for col in 0..grid_layout.cols {
        blocks_still_moving |= animate_column_falling(grid_layout, col, time_delta);
    }

    blocks_still_moving
}

/// Returns true if blocks are still moving
fn animate_column_falling(grid_layout: &mut GridLayout, col: u32, time_delta: f32) -> bool {
    let mut blocks_still_moving = false;
    let mut empty_spaces = 0;

    for row in (0..grid_layout.rows).rev() {
        let original_grid_position = coordinate(row, col);
        if grid_layout.is_empty_at(original_grid_position) {
            empty_spaces += 1;
        } else if empty_spaces > 0 {
            if let Some(mut block) = grid_layout.take_block(original_grid_position) {
                let terminal_grid_position = original_grid_position + coordinate(empty_spaces, 0);
                let terminal_world_position = grid_layout.grid_to_world(terminal_grid_position);
                apply_force(&mut block, vec2(0.0, FORCE), time_delta);

                if block.position.y >= terminal_world_position.y {
                    block.position = terminal_world_position;
                    block.velocity = Vec2::ZERO;
                    grid_layout.place_block(terminal_grid_position, block);
                } else {
                    blocks_still_moving = true;
                    // Put the block back, its not in its final location yet
                    grid_layout.place_block(original_grid_position, block);
                }
            }
        }
    }

    blocks_still_moving
}

/// Returns true if columns are still moving
pub fn animate_columns_shifting(grid_layout: &mut GridLayout, time_delta: f32) -> bool {
    let mut columns_still_moving = false;
    let mut empty_columns = 0;

    for col in 0..grid_layout.cols {
        if grid_layout.is_column_empty(col) {
            empty_columns += 1;
            continue;
        }

        columns_still_moving |= animate_column_shift(grid_layout, col, empty_columns, time_delta);
    }

    columns_still_moving
}

/// Returns true if column is still moving
fn animate_column_shift(
    grid_layout: &mut GridLayout,
    col: u32,
    number_of_columns: u32,
    time_delta: f32,
) -> bool {
    let mut column_still_moving = false;

    for row in 0..grid_layout.rows {
        let original_grid_position = coordinate(row, col);
        let terminal_grid_position = original_grid_position - coordinate(0, number_of_columns);
        let terminal_world_position = grid_layout.grid_to_world(terminal_grid_position);

        if let Some(mut block) = grid_layout.take_block(original_grid_position) {
            apply_force(&mut block, vec2(-FORCE, 0.0), time_delta);

            if block.position.x <= terminal_world_position.x {
                block.position = terminal_world_position;
                block.velocity = Vec2::ZERO;
                grid_layout.place_block(terminal_grid_position, block);
            } else {
                column_still_moving = true;
                // Put the block back, its not in its final location yet
                grid_layout.place_block(original_grid_position, block);
            }
        }
    }

    column_still_moving
}
