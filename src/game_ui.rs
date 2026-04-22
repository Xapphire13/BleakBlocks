use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Rect,
    miniquad::window::set_mouse_cursor,
    shapes::draw_rectangle,
    text::{
        Font, TextDimensions, TextParams, draw_text_ex, load_ttf_font_from_bytes, measure_text,
    },
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    app::{AppState, UiContext},
    constants::{
        style::{BACKGROUND_COLOR, BLOCK_INSET, GRID_BACKGROUND_COLOR},
        ui::{
            BODY_TEXT_SIZE, BUTTON_BACKGROUND_COLOR, BUTTON_PADDING, BUTTON_SHADOW_COLOR,
            CARD_BORDER_COLOR, CONTAINER_INNER_PADDING, CORNER_RADIUS, LABEL_TEXT_COLOR,
            LABEL_TEXT_SIZE, LABEL_VALUE_GAP, LABEL_VALUE_SIZE, PAUSE_ICON_SIZE,
            PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_HOVER_COLOR, PRIMARY_BUTTON_SHADOW_COLOR,
            TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING,
        },
    },
    drawing::draw_rounded_rect,
};

pub struct GameUi {
    title_font: Font,
    body_font: Font,
    buttons: Vec<Button>,
    status_panel_height: f32,
}

impl GameUi {
    pub fn new(app_state: AppState) -> Self {
        let title_font =
            load_ttf_font_from_bytes(include_bytes!("../assets/Creepster-Regular.ttf")).unwrap();
        let body_font =
            load_ttf_font_from_bytes(include_bytes!("../assets/Jersey15-Regular.ttf")).unwrap();

        let label_dims = measure_text("A", Some(&body_font), BODY_TEXT_SIZE, 1.0);
        let value_dims = measure_text("A", Some(&title_font), LABEL_VALUE_SIZE, 1.0);
        let status_panel_height = WINDOW_PADDING.y * 2.0
            + CONTAINER_INNER_PADDING * 2.0
            + label_dims.height
            + LABEL_VALUE_GAP
            + value_dims.height;

        let mut game_ui = Self {
            title_font,
            body_font,
            buttons: vec![],
            status_panel_height,
        };

        game_ui.update_buttons(app_state, false);

        game_ui
    }

    pub fn status_panel_height(&self) -> f32 {
        self.status_panel_height
    }

    pub fn render(&self, ctx: UiContext) {
        set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);

        match ctx.state {
            AppState::Playing => self.render_status_panel(ctx.blocks_remaining, ctx.score),
            AppState::GameOver => self.render_game_over(ctx.score),
            AppState::MainMenu => self.render_main_menu(),
        }

        for button in &self.buttons {
            self.render_button(button);
        }
    }

    pub fn handle_input(&self) -> Option<ButtonId> {
        self.buttons
            .iter()
            .find(|button| button.is_pressed())
            .map(|button| button.id.clone())
    }

    pub fn update_buttons(&mut self, app_state: AppState, is_existing_game: bool) {
        self.buttons = match app_state {
            AppState::Playing => {
                let panel_y = screen_height() - self.status_panel_height;
                let card_h = self.status_panel_height - WINDOW_PADDING.y * 2.0;
                let pause_label = "||";
                let pause_dims =
                    measure_text(pause_label, Some(&self.title_font), PAUSE_ICON_SIZE, 1.0);
                // Square face matching card height; shadow extends below the face
                let btn_w = card_h;
                let btn_h = card_h + BLOCK_INSET;
                let btn_x = screen_width() - WINDOW_PADDING.x - btn_w;
                let btn_y = panel_y + WINDOW_PADDING.y;
                vec![Button::new(
                    ButtonId::Pause,
                    Rect::new(btn_x, btn_y, btn_w, btn_h),
                    pause_label.to_string(),
                    pause_dims,
                    PAUSE_ICON_SIZE,
                    false,
                )]
            }
            AppState::GameOver => {
                let y = screen_height() - WINDOW_PADDING.y;
                self.layout_buttons(&[("Menu", ButtonId::Menu, false)], y)
            }
            AppState::MainMenu => {
                let mut items: Vec<(&str, ButtonId, bool)> = vec![];
                if is_existing_game {
                    items.push(("Resume", ButtonId::Resume, true));
                    items.push(("New game", ButtonId::NewGame, false));
                } else {
                    items.push(("New game", ButtonId::NewGame, true));
                }
                items.push(("Settings", ButtonId::Settings, false));
                items.push(("High scores", ButtonId::HighScores, false));
                self.layout_buttons(&items, 125.0)
            }
        };
    }

    fn layout_buttons(&self, items: &[(&str, ButtonId, bool)], start_y: f32) -> Vec<Button> {
        let measurements: Vec<TextDimensions> = items
            .iter()
            .map(|(text, _, _)| measure_text(text, Some(&self.title_font), BODY_TEXT_SIZE, 1.0))
            .collect();

        let max_btn_w = measurements
            .iter()
            .map(|d| d.width + 2.0 * BUTTON_PADDING.x)
            .fold(0.0f32, f32::max);

        let center_x = screen_width() / 2.0;
        let mut y = start_y;
        let mut buttons = Vec::with_capacity(items.len());

        for ((text, id, is_primary), dims) in items.iter().zip(measurements.iter()) {
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
                *is_primary,
            ));
            y += face_h + BLOCK_INSET + 8.0;
        }

        buttons
    }

    fn render_status_panel(&self, blocks_remaining: u32, score: u32) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let panel_y = screen_height - self.status_panel_height;

        draw_rectangle(
            0.0,
            panel_y,
            screen_width,
            self.status_panel_height,
            GRID_BACKGROUND_COLOR,
        );

        let card_y = panel_y + WINDOW_PADDING.y;
        let card_h = self.status_panel_height - WINDOW_PADDING.y * 2.0;
        let mut card_x = WINDOW_PADDING.x;

        card_x = self.render_datum_card(
            card_x,
            card_y,
            card_h,
            "Blocks left",
            &blocks_remaining.to_formatted_string(&Locale::en),
        );
        card_x += WINDOW_PADDING.x;

        self.render_datum_card(
            card_x,
            card_y,
            card_h,
            "Score",
            &score.to_formatted_string(&Locale::en),
        );
    }

    fn render_datum_card(&self, x: f32, y: f32, h: f32, label: &str, value: &str) -> f32 {
        let label_upper = label.to_uppercase();
        let label_dims = measure_text(&label_upper, Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        let value_dims = measure_text(value, Some(&self.title_font), LABEL_VALUE_SIZE, 1.0);
        let content_w = f32::max(label_dims.width, value_dims.width);
        let card_w = content_w + CONTAINER_INNER_PADDING * 2.0;

        draw_rounded_rect(x, y, card_w, h, CORNER_RADIUS, CARD_BORDER_COLOR);
        draw_rounded_rect(
            x + 1.0,
            y + 1.0,
            card_w - 2.0,
            h - 2.0,
            CORNER_RADIUS - 1.0,
            BACKGROUND_COLOR,
        );

        draw_text_ex(
            &label_upper,
            x + CONTAINER_INNER_PADDING,
            y + CONTAINER_INNER_PADDING + label_dims.offset_y,
            TextParams {
                font_size: LABEL_TEXT_SIZE,
                color: LABEL_TEXT_COLOR,
                font: Some(&self.body_font),
                ..Default::default()
            },
        );

        let value_y =
            y + CONTAINER_INNER_PADDING + label_dims.height + LABEL_VALUE_GAP + value_dims.offset_y;
        draw_text_ex(
            value,
            x + CONTAINER_INNER_PADDING,
            value_y,
            TextParams {
                font_size: LABEL_VALUE_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.title_font),
                ..Default::default()
            },
        );

        x + card_w
    }

    fn render_game_over(&self, score: u32) {
        let screen_width = screen_width();
        let screen_height = screen_height();

        let text = "Game Over!";
        let dimensions = measure_text(text, Some(&self.title_font), TITLE_TEXT_SIZE, 1.0);
        let y = (screen_height - dimensions.height) / 2.0;
        draw_text_ex(
            text,
            (screen_width - dimensions.width) / 2.0,
            y,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.title_font),
                ..Default::default()
            },
        );

        let text = format!("Score: {}", score.to_formatted_string(&Locale::en));
        let dimensions = measure_text(&text, Some(&self.body_font), BODY_TEXT_SIZE, 1.0);
        draw_text_ex(
            &text,
            (screen_width - dimensions.width) / 2.0,
            y + dimensions.height + 8.0,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.body_font),
                ..Default::default()
            },
        );
    }

    fn render_main_menu(&self) {
        let text = "Bleak Blocks";
        let dimensions = measure_text(text, Some(&self.title_font), TITLE_TEXT_SIZE, 1.0);
        draw_text_ex(
            text,
            (screen_width() - dimensions.width) / 2.0,
            WINDOW_PADDING.y + dimensions.height,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.title_font),
                ..Default::default()
            },
        );
    }

    fn render_button(&self, button: &Button) {
        let (border_color, fill_color, hover_color, shadow_color) = if button.is_primary {
            (
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_HOVER_COLOR,
                PRIMARY_BUTTON_SHADOW_COLOR,
            )
        } else {
            (
                CARD_BORDER_COLOR,
                BUTTON_BACKGROUND_COLOR,
                CARD_BORDER_COLOR,
                BUTTON_SHADOW_COLOR,
            )
        };

        let fill = if button.is_hovered() {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
            hover_color
        } else {
            fill_color
        };

        let face_h = button.bounds.h - BLOCK_INSET;

        // Shadow strip (full height)
        draw_rounded_rect(
            button.bounds.x,
            button.bounds.y,
            button.bounds.w,
            button.bounds.h,
            CORNER_RADIUS,
            shadow_color,
        );
        // Border (face height only, so shadow strip shows at bottom)
        draw_rounded_rect(
            button.bounds.x,
            button.bounds.y,
            button.bounds.w,
            face_h,
            CORNER_RADIUS,
            border_color,
        );
        // Fill (1px inset from border)
        draw_rounded_rect(
            button.bounds.x + 1.0,
            button.bounds.y + 1.0,
            button.bounds.w - 2.0,
            face_h - 2.0,
            CORNER_RADIUS - 1.0,
            fill,
        );

        let face_center_y = button.bounds.y + face_h / 2.0;
        draw_text_ex(
            &button.label,
            button.bounds.center().x - button.label_dimensions.width / 2.0,
            face_center_y - button.label_dimensions.height / 2.0 + button.label_dimensions.offset_y,
            TextParams {
                font_size: button.font_size,
                color: TEXT_COLOR,
                font: Some(&self.title_font),
                ..Default::default()
            },
        );
    }
}

#[derive(PartialEq, Clone)]
pub enum ButtonId {
    Menu,
    NewGame,
    Pause,
    Resume,
    Settings,
    HighScores,
}

pub struct Button {
    id: ButtonId,
    bounds: Rect,
    label: String,
    label_dimensions: TextDimensions,
    font_size: u16,
    is_primary: bool,
}

impl Button {
    fn new(
        id: ButtonId,
        bounds: Rect,
        label: String,
        label_dimensions: TextDimensions,
        font_size: u16,
        is_primary: bool,
    ) -> Self {
        Self {
            id,
            bounds,
            label,
            label_dimensions,
            font_size,
            is_primary,
        }
    }

    fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}
