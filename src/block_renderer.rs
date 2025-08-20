use std::collections::HashSet;

use crate::{
    block::{Block, BlockState, BlockType},
    coordinate::{Coordinate, coordinate},
    grid_layout::GridLayout,
    sprite_sheet::{SpriteId, SpriteSheet},
};

pub fn render_blocks(
    grid_layout: &GridLayout,
    sprite_sheet: &SpriteSheet,
    hovered_blocks: HashSet<Coordinate>,
) {
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

                render_block(sprite_sheet, block, block_state);
            }
        }
    }
}

fn render_block(sprite_sheet: &SpriteSheet, block: &Block, state: BlockState) {
    sprite_sheet.render_sprite(
        get_sprite_id(&block.block_type),
        block.position,
        block.size,
        match state {
            BlockState::Default => 1.0,
            BlockState::Hover => 0.5,
        },
    );
}

fn get_sprite_id(block_type: &BlockType) -> SpriteId {
    match block_type {
        BlockType::Brain => SpriteId(0, 0),
        BlockType::Blood => SpriteId(0, 1),
        BlockType::Poison => SpriteId(0, 2),
        BlockType::Ghost => SpriteId(0, 3),
        BlockType::Gravestone => SpriteId(1, 0),
        BlockType::Flame => SpriteId(1, 1),
        BlockType::Potion => SpriteId(1, 2),
        BlockType::Coffin => SpriteId(1, 3),
    }
}
