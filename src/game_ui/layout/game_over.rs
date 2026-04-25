use macroquad::text::{Font, TextParams, draw_text_ex, measure_text};
use macroquad::window::{screen_height, screen_width};
use num_format::{Locale, ToFormattedString};

use crate::constants::ui::{BODY_TEXT_SIZE, TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};
use super::compute_button_stack;

pub(in super::super) struct GameOverLayout {
    pub(super) buttons: Vec<Button>,
}

impl GameOverLayout {
    pub(in super::super) fn compute(title_font: &Font) -> Self {
        let y = screen_height() - WINDOW_PADDING.y;
        Self {
            buttons: compute_button_stack(
                title_font,
                &[("Menu", ButtonId::Menu, ButtonStyle::Secondary)],
                y,
            ),
        }
    }

    pub(super) fn render(&self, fonts: Fonts, score: u32) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let text = "Game Over!";
        let dims = measure_text(text, Some(fonts.title), TITLE_TEXT_SIZE, 1.0);
        let y = (screen_h - dims.height) / 2.0;
        draw_text_ex(
            text,
            (screen_w - dims.width) / 2.0,
            y,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(fonts.title),
                ..Default::default()
            },
        );

        let score_text = format!("Score: {}", score.to_formatted_string(&Locale::en));
        let score_dims = measure_text(&score_text, Some(fonts.body), BODY_TEXT_SIZE, 1.0);
        draw_text_ex(
            &score_text,
            (screen_w - score_dims.width) / 2.0,
            y + score_dims.height + 8.0,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );
    }
}
