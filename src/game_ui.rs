use macroquad::{
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    constants::ui::{BODY_TEXT_COLOR, BODY_TEXT_SIZE, PADDING_X, PADDING_Y},
    game::Game,
};

pub struct GameUi {}

impl GameUi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, game: &Game) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        // Blocks remaining
        let text = format!(
            "Blocks remaining: {}",
            game.blocks_remaining().to_formatted_string(&Locale::en)
        );
        let x = PADDING_X;
        let y = screen_height - PADDING_Y;
        draw_text(&text, x, y, BODY_TEXT_SIZE, BODY_TEXT_COLOR);

        // Menu button
        // TODO

        // Score
        let text = format!("Score: {}", game.score().to_formatted_string(&Locale::en));
        let text_dimensions = measure_text(&text, None, BODY_TEXT_SIZE as u16, 1.0);
        let x = screen_width - PADDING_X - text_dimensions.width;
        let y = screen_height - PADDING_Y;
        draw_text(&text, x, y, BODY_TEXT_SIZE, BODY_TEXT_COLOR);
    }
}
