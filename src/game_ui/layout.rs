mod game_over;
mod main_menu;
mod playing;
mod settings;

use macroquad::{
    math::Rect,
    text::{Font, TextDimensions, measure_text},
    window::screen_width,
};

use crate::constants::{
    style::BLOCK_INSET,
    ui::{BODY_TEXT_SIZE, BUTTON_PADDING},
};

use super::Fonts;
use super::buttons::{Button, ButtonId, ButtonStyle};

pub use game_over::GameOverLayout;
pub use main_menu::MainMenuLayout;
pub use playing::PlayingLayout;
pub use settings::SettingsLayout;

pub enum ScreenLayout {
    Playing(PlayingLayout),
    MainMenu(MainMenuLayout),
    GameOver(GameOverLayout),
    Settings(SettingsLayout),
}

impl Default for ScreenLayout {
    fn default() -> Self {
        ScreenLayout::MainMenu(MainMenuLayout { buttons: vec![] })
    }
}

impl ScreenLayout {
    pub fn buttons(&self) -> &[Button] {
        match self {
            ScreenLayout::Playing(l) => &l.buttons,
            ScreenLayout::MainMenu(l) => &l.buttons,
            ScreenLayout::GameOver(l) => &l.buttons,
            ScreenLayout::Settings(l) => &l.buttons,
        }
    }

    pub fn render(&self, fonts: Fonts, blocks_remaining: u32, score: u32) {
        match self {
            ScreenLayout::Playing(l) => l.render(fonts, blocks_remaining, score),
            ScreenLayout::GameOver(l) => l.render(fonts, score),
            ScreenLayout::MainMenu(l) => l.render(fonts),
            ScreenLayout::Settings(l) => l.render(fonts),
        }
    }

    pub fn status_panel_height(&self) -> f32 {
        match self {
            ScreenLayout::Playing(l) => l.status_panel_height,
            _ => unreachable!("status_panel_height() must only be called in Playing state"),
        }
    }
}

pub fn compute_button_stack(
    title_font: &Font,
    items: &[(&str, ButtonId, ButtonStyle)],
    start_y: f32,
) -> Vec<Button> {
    let measurements: Vec<TextDimensions> = items
        .iter()
        .map(|(text, _, _)| measure_text(text, Some(title_font), BODY_TEXT_SIZE, 1.0))
        .collect();

    let max_btn_w = measurements
        .iter()
        .map(|d| d.width + 2.0 * BUTTON_PADDING.x)
        .fold(0.0f32, f32::max);

    let center_x = screen_width() / 2.0;
    let mut y = start_y;
    let mut buttons = Vec::with_capacity(items.len());

    for (item, dims) in items.iter().zip(measurements.iter()) {
        let (text, id, style): &(&str, ButtonId, ButtonStyle) = item;
        let face_h = dims.height + 2.0 * BUTTON_PADDING.y;
        let bounds = Rect::new(
            center_x - max_btn_w / 2.0,
            y - dims.offset_y - BUTTON_PADDING.y,
            max_btn_w,
            face_h + BLOCK_INSET,
        );
        buttons.push(Button::new(
            id.clone(),
            bounds,
            text.to_string(),
            *dims,
            BODY_TEXT_SIZE,
            style.clone(),
        ));
        y += face_h + BLOCK_INSET + 8.0;
    }

    buttons
}
