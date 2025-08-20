use macroquad::{
    color::Color,
    math::{Rect, Vec2},
    prelude::ImageFormat,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

#[derive(Debug)]
pub struct SpriteId(pub u32, pub u32);

pub struct SpriteSheet {
    sprite_sheet: Texture2D,
    rows: u32,
    cols: u32,
    sprite_size: f32,
}

impl SpriteSheet {
    pub fn new(bytes: &[u8], rows: u32, cols: u32, size: f32) -> Self {
        Self {
            sprite_sheet: Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png)),
            rows,
            cols,
            sprite_size: size,
        }
    }

    pub fn render_sprite(&self, sprite_id: SpriteId, position: Vec2, size: f32, alpha: f32) {
        let SpriteId(sprite_row, sprite_col) = sprite_id;

        if sprite_col >= self.cols || sprite_row >= self.rows {
            panic!("Invalid sprite ID, {sprite_id:?}");
        }

        let color = Color::new(1.0, 1.0, 1.0, alpha);

        draw_texture_ex(
            &self.sprite_sheet,
            position.x,
            position.y,
            color,
            DrawTextureParams {
                source: Some(Rect::new(
                    sprite_col as f32 * self.sprite_size,
                    sprite_row as f32 * self.sprite_size,
                    self.sprite_size,
                    self.sprite_size,
                )),
                dest_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
        );
    }
}
