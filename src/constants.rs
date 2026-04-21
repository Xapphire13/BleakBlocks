pub mod physics {
    /// Force in pixels per second^2 that is applied to moving blocks
    pub const FORCE: f32 = 2000.0;
}

pub mod style {
    use macroquad::color::Color;

    pub const BACKGROUND_COLOR: Color = Color::from_hex(0x31263E);
    pub const GRID_BACKGROUND_COLOR: Color = Color::from_hex(0x1A1226);
    pub const EMPTY_BLOCK_COLOR: Color = Color::from_hex(0x2D2340);
}

pub mod ui {
    use macroquad::{
        color::{Color, WHITE},
        math::{Vec2, vec2},
    };

    // Padding
    pub const WINDOW_PADDING: Vec2 = vec2(8.0, 16.0);
    pub const BUTTON_PADDING: Vec2 = vec2(8.0, 4.0);
    pub const CONTAINER_INNER_PADDING: f32 = 8.0;
    pub const BLOCK_GAP: f32 = 3.0;

    // Text size
    pub const TITLE_TEXT_SIZE: u16 = 64;
    pub const BODY_TEXT_SIZE: u16 = 24;
    pub const LABEL_VALUE_GAP: f32 = 4.0;

    // Shape
    pub const CORNER_RADIUS: f32 = 8.0;

    pub const TEXT_COLOR: Color = WHITE;
}
