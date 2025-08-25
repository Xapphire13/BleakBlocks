use macroquad::{
    color::LIGHTGRAY,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Rect,
    miniquad::window::set_mouse_cursor,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{
        Font, TextDimensions, TextParams, draw_text_ex, load_ttf_font_from_bytes, measure_text,
    },
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    app::{App, AppState},
    constants::ui::{BODY_TEXT_SIZE, BUTTON_PADDING, TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING},
};

pub struct GameUi {
    font: Font,
    buttons: Vec<Button>,
}

impl GameUi {
    pub fn new(app_state: AppState) -> Self {
        let mut game_ui = Self {
            font: load_ttf_font_from_bytes(include_bytes!("../assets/GrenzeGotisch-Regular.ttf"))
                .unwrap(),
            buttons: vec![],
        };

        // Initial state
        game_ui.on_game_state_changed(app_state, false);

        game_ui
    }

    pub fn render(&self, game: &App) {
        set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);

        match game.state() {
            AppState::Playing => self.render_overlay(game),
            AppState::GameOver => self.render_game_over(game),
            AppState::MainMenu => self.render_main_menu(),
        }
    }

    pub fn handle_input(&self) -> Option<ButtonId> {
        if let Some(pressed_button) = self.buttons.iter().find(|button| button.is_pressed()) {
            return Some(pressed_button.id.clone());
        }

        None
    }

    pub fn on_game_state_changed(&mut self, app_state: AppState, is_existing_game: bool) {
        match app_state {
            AppState::Playing => {
                self.buttons = vec![{
                    let text: &str = "Menu";
                    let text_dimensions = measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
                    let x = (screen_width() - text_dimensions.width) / 2.0;
                    let y = screen_height() - WINDOW_PADDING.y;
                    Button::new(
                        ButtonId::Menu,
                        Rect::new(
                            x - BUTTON_PADDING.x,
                            y - text_dimensions.offset_y - BUTTON_PADDING.y,
                            text_dimensions.width + 2.0 * BUTTON_PADDING.x,
                            text_dimensions.height + 2.0 * BUTTON_PADDING.y,
                        ),
                        text.to_owned(),
                        text_dimensions,
                    )
                }]
            }
            AppState::MainMenu => {
                let mut buttons = vec![];

                if is_existing_game {
                    buttons.push({
                        let text: &str = "Resume";
                        let text_dimensions =
                            measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
                        let x = (screen_width() - text_dimensions.width) / 2.0;
                        let y = 100.0;
                        Button::new(
                            ButtonId::Resume,
                            Rect::new(
                                x - BUTTON_PADDING.x,
                                y - text_dimensions.offset_y - BUTTON_PADDING.y,
                                text_dimensions.width + 2.0 * BUTTON_PADDING.x,
                                text_dimensions.height + 2.0 * BUTTON_PADDING.y,
                            ),
                            text.to_owned(),
                            text_dimensions,
                        )
                    });
                }

                buttons.push({
                    let text: &str = "New game";
                    let text_dimensions = measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
                    let x = (screen_width() - text_dimensions.width) / 2.0;
                    let y = if !is_existing_game { 100.0 } else { 150.0 };
                    Button::new(
                        ButtonId::NewGame,
                        Rect::new(
                            x - BUTTON_PADDING.x,
                            y - text_dimensions.offset_y - BUTTON_PADDING.y,
                            text_dimensions.width + 2.0 * BUTTON_PADDING.x,
                            text_dimensions.height + 2.0 * BUTTON_PADDING.y,
                        ),
                        text.to_owned(),
                        text_dimensions,
                    )
                });
                buttons.push({
                    let text: &str = "Settings";
                    let text_dimensions = measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
                    let x = (screen_width() - text_dimensions.width) / 2.0;
                    let y = if !is_existing_game { 150.0 } else { 200.0 };
                    Button::new(
                        ButtonId::Settings,
                        Rect::new(
                            x - BUTTON_PADDING.x,
                            y - text_dimensions.offset_y - BUTTON_PADDING.y,
                            text_dimensions.width + 2.0 * BUTTON_PADDING.x,
                            text_dimensions.height + 2.0 * BUTTON_PADDING.y,
                        ),
                        text.to_owned(),
                        text_dimensions,
                    )
                });
                buttons.push({
                    let text: &str = "High scores";
                    let text_dimensions = measure_text(text, Some(&self.font), BODY_TEXT_SIZE, 1.0);
                    let x = (screen_width() - text_dimensions.width) / 2.0;
                    let y = if !is_existing_game { 200.0 } else { 250.0 };
                    Button::new(
                        ButtonId::HighScores,
                        Rect::new(
                            x - BUTTON_PADDING.x,
                            y - text_dimensions.offset_y - BUTTON_PADDING.y,
                            text_dimensions.width + 2.0 * BUTTON_PADDING.x,
                            text_dimensions.height + 2.0 * BUTTON_PADDING.y,
                        ),
                        text.to_owned(),
                        text_dimensions,
                    )
                });

                self.buttons = buttons;
            }
            AppState::GameOver => self.buttons = vec![],
        }
    }

    fn render_overlay(&self, game: &App) {
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
        if let Some(menu_button) = &self
            .buttons
            .iter()
            .find(|button| button.id == ButtonId::Menu)
        {
            self.render_button(menu_button);
        }

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

    fn render_game_over(&self, game: &App) {
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

    fn render_main_menu(&self) {
        let text = "Bleak Blocks";
        let dimensions = measure_text(text, Some(&self.font), TITLE_TEXT_SIZE, 1.0);
        let y = WINDOW_PADDING.y + dimensions.height;
        draw_text_ex(
            text,
            (screen_width() - dimensions.width) / 2.0,
            y,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );

        for menu_item in &self.buttons {
            self.render_button(menu_item);
        }
    }

    fn render_button(&self, button: &Button) {
        if button.is_hovered() {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
            draw_rectangle(
                button.bounds.x,
                button.bounds.y,
                button.bounds.w,
                button.bounds.h,
                LIGHTGRAY,
            );
        }
        draw_rectangle_lines(
            button.bounds.x,
            button.bounds.y,
            button.bounds.w,
            button.bounds.h,
            1.0,
            TEXT_COLOR,
        );
        draw_text_ex(
            &button.label,
            button.bounds.center().x - button.label_dimensions.width / 2.0,
            button.bounds.bottom() - BUTTON_PADDING.y - button.label_dimensions.height
                + button.label_dimensions.offset_y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.font),
                ..Default::default()
            },
        );
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonId {
    Menu,
    NewGame,
    Resume,
    Settings,
    HighScores,
}

pub struct Button {
    id: ButtonId,
    bounds: Rect,
    label: String,
    label_dimensions: TextDimensions,
}

impl Button {
    fn new(id: ButtonId, bounds: Rect, label: String, label_dimensions: TextDimensions) -> Self {
        Self {
            id,
            bounds,
            label,
            label_dimensions,
        }
    }

    fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}
