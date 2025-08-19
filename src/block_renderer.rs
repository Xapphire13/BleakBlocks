use std::collections::HashSet;

use crate::{
    block::BlockState,
    coordinate::{Coordinate, coordinate},
    grid_layout::GridLayout,
};

pub fn render_blocks(grid_layout: &GridLayout, hovered_blocks: HashSet<Coordinate>) {
    // Render blocks
    for row in 0..grid_layout.rows {
        for col in 0..grid_layout.cols {
            let position = coordinate(row, col);
            if let Some(block) = grid_layout.get_block_at_grid_position(position) {
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
