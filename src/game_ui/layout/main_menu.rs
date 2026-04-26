use macroquad::text::{Font, TextParams, draw_text_ex, measure_text};
use macroquad::window::screen_width;

use crate::constants::ui::{CHROME_HEIGHT, TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};
use super::compute_button_stack;

pub struct MainMenuLayout {
    pub buttons: Vec<Button>,
}

impl MainMenuLayout {
    pub fn compute(title_font: &Font, is_existing_game: bool) -> Self {
        let mut items: Vec<(&str, ButtonId, ButtonStyle)> = vec![];
        if is_existing_game {
            items.push(("Resume", ButtonId::Resume, ButtonStyle::Primary));
            items.push(("New game", ButtonId::NewGame, ButtonStyle::Secondary));
        } else {
            items.push(("New game", ButtonId::NewGame, ButtonStyle::Primary));
        }
        items.push(("Settings", ButtonId::Settings, ButtonStyle::Secondary));
        items.push(("High scores", ButtonId::HighScores, ButtonStyle::Secondary));

        Self {
            buttons: compute_button_stack(title_font, &items, CHROME_HEIGHT + 125.0),
        }
    }

    pub fn render(&self, fonts: Fonts) {
        let text = "Bleak Blocks";
        let dims = measure_text(text, Some(fonts.title), TITLE_TEXT_SIZE, 1.0);
        draw_text_ex(
            text,
            (screen_width() - dims.width) / 2.0,
            CHROME_HEIGHT + WINDOW_PADDING.y + dims.height,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(fonts.title),
                ..Default::default()
            },
        );
    }
}
