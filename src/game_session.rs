use crate::{
    difficulty::Difficulty, grid_layout::GridLayout, grid_size::GridSize,
    physics_system::PhysicsSystem,
};

pub enum GameState {
    Playing,
    BlocksFalling,
    ColumnsShifting,
}

pub struct GameSession {
    pub state: GameState,
    pub score: u32,
    pub grid_size: GridSize,
    pub difficulty: Difficulty,
    pub layout: GridLayout,
    pub physics_system: PhysicsSystem,
}

impl GameSession {
    pub fn is_game_over(&self) -> bool {
        self.layout.blocks_remaining == 0
    }

    pub fn blocks_remaining(&self) -> u32 {
        self.layout.blocks_remaining
    }
}
