pub mod physics {
    /// Force in pixels per second^2 that is applied to moving blocks
    pub const FORCE: f32 = 2000.0;
}

pub mod style {
    use macroquad::color::Color;

    pub const BACKGROUND_COLOR: Color = Color::from_hex(0x31263E);
    pub const GRID_BACKGROUND_COLOR: Color = Color::from_hex(0x1A1226);
    pub const EMPTY_BLOCK_COLOR: Color = Color::from_hex(0x2D2340);
    pub const BLOCK_SHADOW_FACTOR: f32 = 0.6;
    pub const BLOCK_INSET: f32 = 3.0;
}

pub mod ui {
    use macroquad::{
        color::{Color, WHITE},
        math::{Vec2, vec2},
    };

    // Padding
    pub const WINDOW_PADDING: Vec2 = vec2(8.0, 8.0);
    pub const BUTTON_PADDING: Vec2 = vec2(8.0, 4.0);
    pub const CONTAINER_INNER_PADDING: f32 = 8.0;
    pub const BLOCK_GAP: f32 = 3.0;

    // Text size
    pub const TITLE_TEXT_SIZE: u16 = 64;
    pub const BODY_TEXT_SIZE: u16 = 24;
    pub const LABEL_TEXT_SIZE: u16 = 16;
    pub const LABEL_VALUE_SIZE: u16 = 32;
    pub const LABEL_VALUE_GAP: f32 = 10.0;

    // Shape
    pub const CORNER_RADIUS: f32 = 8.0;

    pub const TEXT_COLOR: Color = WHITE;
    pub const LABEL_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.55);
    pub const CARD_BORDER_COLOR: Color = Color::from_hex(0x4E3F63);
}
