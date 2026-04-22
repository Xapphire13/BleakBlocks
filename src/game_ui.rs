use macroquad::{
    math::Rect,
    miniquad::window::set_mouse_cursor,
    text::{Font, load_ttf_font_from_bytes, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    app::{AppState, UiContext},
    constants::{
        style::BLOCK_INSET,
        ui::{
            BODY_TEXT_SIZE, CONTAINER_INNER_PADDING, LABEL_VALUE_GAP, LABEL_VALUE_SIZE,
            PAUSE_ICON_SIZE, WINDOW_PADDING,
        },
    },
    difficulty::Difficulty,
    grid_size::GridSize,
};

mod buttons;
mod layout;
mod rendering;

pub use buttons::ButtonId;
use buttons::{Button, ToggleButton};

pub struct GameUi {
    title_font: Font,
    body_font: Font,
    buttons: Vec<Button>,
    toggle_buttons: Vec<ToggleButton>,
    status_panel_height: f32,
    settings_grid_size_label_y: f32,
    settings_difficulty_label_y: f32,
}

impl GameUi {
    pub fn new() -> Self {
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

        Self {
            title_font,
            body_font,
            buttons: vec![],
            toggle_buttons: vec![],
            status_panel_height,
            settings_grid_size_label_y: 0.0,
            settings_difficulty_label_y: 0.0,
        }
    }

    pub fn status_panel_height(&self) -> f32 {
        self.status_panel_height
    }

    pub fn render(&self, ctx: UiContext) {
        set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);

        match ctx.state {
            AppState::Playing => rendering::render_status_panel(
                &self.title_font,
                &self.body_font,
                self.status_panel_height,
                ctx.blocks_remaining,
                ctx.score,
            ),
            AppState::GameOver => {
                rendering::render_game_over(&self.title_font, &self.body_font, ctx.score)
            }
            AppState::MainMenu => rendering::render_main_menu(&self.title_font),
            AppState::Settings => rendering::render_settings(
                &self.title_font,
                &self.body_font,
                self.settings_grid_size_label_y,
                self.settings_difficulty_label_y,
            ),
        }

        for button in &self.buttons {
            rendering::render_button(&self.title_font, button);
        }

        for toggle in &self.toggle_buttons {
            rendering::render_toggle_button(&self.title_font, &self.body_font, toggle);
        }
    }

    pub fn handle_input(&self) -> Option<ButtonId> {
        self.buttons
            .iter()
            .find(|button| button.is_pressed())
            .map(|button| button.id.clone())
            .or_else(|| {
                self.toggle_buttons
                    .iter()
                    .find(|toggle| toggle.is_pressed())
                    .map(|toggle| toggle.id.clone())
            })
    }

    pub fn update_buttons(
        &mut self,
        app_state: AppState,
        is_existing_game: bool,
        grid_size: GridSize,
        difficulty: Difficulty,
    ) {
        self.toggle_buttons.clear();

        match app_state {
            AppState::Playing => {
                let panel_y = screen_height() - self.status_panel_height;
                let card_h = self.status_panel_height - WINDOW_PADDING.y * 2.0;
                let pause_label = "||";
                let pause_dims =
                    measure_text(pause_label, Some(&self.title_font), PAUSE_ICON_SIZE, 1.0);
                let btn_w = card_h;
                let btn_h = card_h + BLOCK_INSET;
                let btn_x = screen_width() - WINDOW_PADDING.x - btn_w;
                let btn_y = panel_y + WINDOW_PADDING.y;
                self.buttons = vec![Button::new(
                    ButtonId::Pause,
                    Rect::new(btn_x, btn_y, btn_w, btn_h),
                    pause_label.to_string(),
                    pause_dims,
                    PAUSE_ICON_SIZE,
                    false,
                )];
            }
            AppState::GameOver => {
                let y = screen_height() - WINDOW_PADDING.y;
                self.buttons = layout::compute_button_stack(
                    &self.title_font,
                    &[("Menu", ButtonId::Menu, false)],
                    y,
                );
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
                self.buttons = layout::compute_button_stack(&self.title_font, &items, 125.0);
            }
            AppState::Settings => {
                let (buttons, toggle_buttons, gs_label_y, diff_label_y) =
                    layout::compute_settings_layout(
                        &self.title_font,
                        &self.body_font,
                        grid_size,
                        difficulty,
                    );
                self.buttons = buttons;
                self.toggle_buttons = toggle_buttons;
                self.settings_grid_size_label_y = gs_label_y;
                self.settings_difficulty_label_y = diff_label_y;
            }
        }
    }
}
