use macroquad::{
    color::LIGHTGRAY,
    input::mouse_position,
    math::Rect,
    miniquad::window::set_mouse_cursor,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{Font, TextParams, draw_text_ex, load_ttf_font_from_bytes, measure_text},
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    constants::ui::{BODY_TEXT_SIZE, BUTTON_PADDING, TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING},
    game::{Game, GameState},
};

pub struct GameUi {
    font: Font,
}

impl GameUi {
    pub fn new() -> Self {
        Self {
            font: load_ttf_font_from_bytes(include_bytes!("../assets/GrenzeGotisch-Regular.ttf"))
                .unwrap(),
        }
    }

    pub fn render(&self, game: &Game) {
        match game.state() {
            GameState::GameOver => self.render_game_over(game),
            _ => self.render_overlay(game),
        }
    }

    fn render_overlay(&self, game: &Game) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Blocks remaining
        let text = format!(
            "Blocks remaining: {}",
            game.blocks_remaining().to_formatted_string(&Locale::en)
        );
        let x = WINDOW_PADDING.x;
        let y = screen_height - WINDOW_PADDING.y;
        draw_text_ex(
            &text,
            x,
            y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        // Menu button
        let text: &str = "Menu";
        let text_dimensions = measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
        let x = (screen_width - text_dimensions.width) / 2.0;
        let y = screen_height - WINDOW_PADDING.y;
        let button_rect = Rect::new(
            x - BUTTON_PADDING.x,
            y - text_dimensions.offset_y - BUTTON_PADDING.y,
            text_dimensions.width + 2.0 * BUTTON_PADDING.x,
            text_dimensions.height + 2.0 * BUTTON_PADDING.y,
        );
        if button_rect.contains(mouse_position().into()) {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
            draw_rectangle(
                button_rect.x,
                button_rect.y,
                button_rect.w,
                button_rect.h,
                LIGHTGRAY,
            );
        } else {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);
        }
        draw_rectangle_lines(
            button_rect.x,
            button_rect.y,
            button_rect.w,
            button_rect.h,
            1.0,
            TEXT_COLOR,
        );
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        // Score
        let text = format!("Score: {}", game.score().to_formatted_string(&Locale::en));
        let text_dimensions = measure_text(&text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
        let x = screen_width - WINDOW_PADDING.x - text_dimensions.width;
        let y = screen_height - WINDOW_PADDING.y;
        draw_text_ex(
            &text,
            x,
            y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );
    }

    fn render_game_over(&self, game: &Game) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let text = "Game Over!";
        let dimensions = measure_text(text, Some(&self.font), TITLE_TEXT_SIZE, 1.0);
        let y = (screen_height - dimensions.height) / 2.0;
        draw_text_ex(
            text,
            (screen_width - dimensions.width) / 2.0,
            y,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        let text = format!("Score: {}", game.score().to_formatted_string(&Locale::en));
        let y = y + dimensions.height + 8.0;
        let dimensions = measure_text(&text, Some(&self.font), TITLE_TEXT_SIZE, 1.0);
        draw_text_ex(
            &text,
            (screen_width - dimensions.width) / 2.0,
            y,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );
    }
}
