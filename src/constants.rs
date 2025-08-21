pub mod layout {
    pub const GRID_MARGIN: f32 = 20.0;
}

pub mod physics {
    /// Force in pixels per second^2 that is applied to moving blocks
    pub const FORCE: f32 = 2000.0;
}

pub mod style {
    // Styles
    pub const BACKGROUND_COLOR: u32 = 0x31263E;
}

pub mod ui {
    use macroquad::color::{Color, WHITE};

    pub const PADDING_Y: f32 = 8.0;
    pub const PADDING_X: f32 = 16.0;
    pub const BODY_TEXT_SIZE: f32 = 16.0;
    pub const BODY_TEXT_COLOR: Color = WHITE;
}
