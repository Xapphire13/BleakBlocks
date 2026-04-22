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
    difficulty::Difficulty,
    drawing::draw_rounded_rect,
    grid_size::GridSize,
};

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
            AppState::Playing => self.render_status_panel(ctx.blocks_remaining, ctx.score),
            AppState::GameOver => self.render_game_over(ctx.score),
            AppState::MainMenu => self.render_main_menu(),
            AppState::Settings => self.render_settings(),
        }

        for button in &self.buttons {
            self.render_button(button);
        }

        for toggle in &self.toggle_buttons {
            self.render_toggle_button(toggle);
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
            AppState::Settings => self.layout_settings_buttons(grid_size, difficulty),
        };
    }

    fn layout_settings_buttons(
        &mut self,
        grid_size: GridSize,
        difficulty: Difficulty,
    ) -> Vec<Button> {
        let is_landscape = screen_width() > screen_height();
        let available_w = screen_width() - 2.0 * WINDOW_PADDING.x;
        let btn_gap = WINDOW_PADDING.x;

        // --- Compute starting y below the title ---
        let title_dims = measure_text("Settings", Some(&self.title_font), TITLE_TEXT_SIZE, 1.0);
        let mut current_y = WINDOW_PADDING.y + title_dims.height + 16.0;

        // --- Grid size toggle buttons (2×2 layout) ---
        let gs_label_dims = measure_text("A", Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        self.settings_grid_size_label_y = current_y;
        current_y += gs_label_dims.height + 8.0;

        let gs_btn_w = (available_w - btn_gap) / 2.0;
        let gs_main_dims = measure_text("X-Large", Some(&self.title_font), BODY_TEXT_SIZE, 1.0);
        let gs_sub_dims = measure_text("00×00", Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        let gs_face_h =
            BUTTON_PADDING.y + gs_main_dims.height + 4.0 + gs_sub_dims.height + BUTTON_PADDING.y;
        let gs_btn_h = gs_face_h + BLOCK_INSET;

        let gs_variants = [
            GridSize::Small,
            GridSize::Medium,
            GridSize::Large,
            GridSize::ExtraLarge,
        ];
        for (i, gs) in gs_variants.iter().enumerate() {
            let row = i / 2;
            let col = i % 2;
            let x = WINDOW_PADDING.x + col as f32 * (gs_btn_w + btn_gap);
            let y = current_y + row as f32 * (gs_btn_h + btn_gap);
            let label = gs.label().to_string();
            let sub_label = gs.size_hint(is_landscape);
            let label_dims = measure_text(&label, Some(&self.title_font), BODY_TEXT_SIZE, 1.0);
            let sub_label_dims =
                measure_text(&sub_label, Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
            self.toggle_buttons.push(ToggleButton {
                id: ButtonId::SetGridSize(*gs),
                bounds: Rect::new(x, y, gs_btn_w, gs_btn_h),
                label,
                label_dimensions: label_dims,
                sub_label: Some(sub_label),
                sub_label_dimensions: Some(sub_label_dims),
                is_selected: *gs == grid_size,
            });
        }
        current_y += 2.0 * gs_btn_h + btn_gap;

        // --- Difficulty toggle buttons (1×3 layout) ---
        current_y += 20.0;
        let diff_label_dims = measure_text("A", Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        self.settings_difficulty_label_y = current_y;
        current_y += diff_label_dims.height + 8.0;

        let diff_btn_w = (available_w - 2.0 * btn_gap) / 3.0;
        let diff_main_dims = measure_text("Normal", Some(&self.title_font), BODY_TEXT_SIZE, 1.0);
        let diff_face_h = BUTTON_PADDING.y + diff_main_dims.height + BUTTON_PADDING.y;
        let diff_btn_h = diff_face_h + BLOCK_INSET;

        let diff_variants = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
        for (i, diff) in diff_variants.iter().enumerate() {
            let x = WINDOW_PADDING.x + i as f32 * (diff_btn_w + btn_gap);
            let label = diff.label().to_string();
            let label_dims = measure_text(&label, Some(&self.title_font), BODY_TEXT_SIZE, 1.0);
            self.toggle_buttons.push(ToggleButton {
                id: ButtonId::SetDifficulty(*diff),
                bounds: Rect::new(x, current_y, diff_btn_w, diff_btn_h),
                label,
                label_dimensions: label_dims,
                sub_label: None,
                sub_label_dimensions: None,
                is_selected: *diff == difficulty,
            });
        }
        current_y += diff_btn_h;

        // --- Back button ---
        current_y += 24.0;
        let back_dims = measure_text("Back", Some(&self.title_font), BODY_TEXT_SIZE, 1.0);
        let back_baseline = current_y + back_dims.offset_y + BUTTON_PADDING.y;
        self.layout_buttons(&[("Back", ButtonId::Back, false)], back_baseline)
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

        let pause_btn_size = card_h;
        let pause_btn_x = screen_width - WINDOW_PADDING.x - pause_btn_size;
        let cards_end = pause_btn_x - WINDOW_PADDING.x;
        let card_w = (cards_end - WINDOW_PADDING.x - WINDOW_PADDING.x) / 2.0;

        let mut card_x = WINDOW_PADDING.x;

        card_x = self.render_datum_card(
            card_x,
            card_y,
            card_w,
            card_h,
            "Blocks left",
            &blocks_remaining.to_formatted_string(&Locale::en),
        );
        card_x += WINDOW_PADDING.x;

        self.render_datum_card(
            card_x,
            card_y,
            card_w,
            card_h,
            "Score",
            &score.to_formatted_string(&Locale::en),
        );
    }

    fn render_datum_card(&self, x: f32, y: f32, w: f32, h: f32, label: &str, value: &str) -> f32 {
        let label_upper = label.to_uppercase();
        let label_dims = measure_text(&label_upper, Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        let value_dims = measure_text(value, Some(&self.title_font), LABEL_VALUE_SIZE, 1.0);

        draw_rounded_rect(x, y, w, h, CORNER_RADIUS, CARD_BORDER_COLOR);
        draw_rounded_rect(
            x + 1.0,
            y + 1.0,
            w - 2.0,
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

        x + w
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

    fn render_settings(&self) {
        let text = "Settings";
        let dims = measure_text(text, Some(&self.title_font), TITLE_TEXT_SIZE, 1.0);
        draw_text_ex(
            text,
            (screen_width() - dims.width) / 2.0,
            WINDOW_PADDING.y + dims.height,
            TextParams {
                font_size: TITLE_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(&self.title_font),
                ..Default::default()
            },
        );

        let label_dims = measure_text("A", Some(&self.body_font), LABEL_TEXT_SIZE, 1.0);
        draw_text_ex(
            "GRID SIZE",
            WINDOW_PADDING.x,
            self.settings_grid_size_label_y + label_dims.offset_y,
            TextParams {
                font_size: LABEL_TEXT_SIZE,
                color: LABEL_TEXT_COLOR,
                font: Some(&self.body_font),
                ..Default::default()
            },
        );
        draw_text_ex(
            "DIFFICULTY",
            WINDOW_PADDING.x,
            self.settings_difficulty_label_y + label_dims.offset_y,
            TextParams {
                font_size: LABEL_TEXT_SIZE,
                color: LABEL_TEXT_COLOR,
                font: Some(&self.body_font),
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

    fn render_toggle_button(&self, toggle: &ToggleButton) {
        let (border_color, fill_color, hover_color, shadow_color) = if toggle.is_selected {
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

        let fill = if toggle.is_hovered() {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
            hover_color
        } else {
            fill_color
        };

        let face_h = toggle.bounds.h - BLOCK_INSET;

        draw_rounded_rect(
            toggle.bounds.x,
            toggle.bounds.y,
            toggle.bounds.w,
            toggle.bounds.h,
            CORNER_RADIUS,
            shadow_color,
        );
        draw_rounded_rect(
            toggle.bounds.x,
            toggle.bounds.y,
            toggle.bounds.w,
            face_h,
            CORNER_RADIUS,
            border_color,
        );
        draw_rounded_rect(
            toggle.bounds.x + 1.0,
            toggle.bounds.y + 1.0,
            toggle.bounds.w - 2.0,
            face_h - 2.0,
            CORNER_RADIUS - 1.0,
            fill,
        );

        let face_h = toggle.bounds.h - BLOCK_INSET;
        let center_x = toggle.bounds.center().x;

        if let (Some(sub_label), Some(sub_dims)) = (&toggle.sub_label, &toggle.sub_label_dimensions)
        {
            // Two-line layout: name + size hint
            let total_text_h = toggle.label_dimensions.height + 4.0 + sub_dims.height;
            let text_top = toggle.bounds.y + (face_h - total_text_h) / 2.0;

            draw_text_ex(
                &toggle.label,
                center_x - toggle.label_dimensions.width / 2.0,
                text_top + toggle.label_dimensions.offset_y,
                TextParams {
                    font_size: BODY_TEXT_SIZE,
                    color: TEXT_COLOR,
                    font: Some(&self.title_font),
                    ..Default::default()
                },
            );
            draw_text_ex(
                sub_label,
                center_x - sub_dims.width / 2.0,
                text_top + toggle.label_dimensions.height + 4.0 + sub_dims.offset_y,
                TextParams {
                    font_size: LABEL_TEXT_SIZE,
                    color: TEXT_COLOR,
                    font: Some(&self.body_font),
                    ..Default::default()
                },
            );
        } else {
            // Single-line layout
            let face_center_y = toggle.bounds.y + face_h / 2.0;
            draw_text_ex(
                &toggle.label,
                center_x - toggle.label_dimensions.width / 2.0,
                face_center_y - toggle.label_dimensions.height / 2.0
                    + toggle.label_dimensions.offset_y,
                TextParams {
                    font_size: BODY_TEXT_SIZE,
                    color: TEXT_COLOR,
                    font: Some(&self.title_font),
                    ..Default::default()
                },
            );
        }
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
    Back,
    SetGridSize(GridSize),
    SetDifficulty(Difficulty),
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

struct ToggleButton {
    id: ButtonId,
    bounds: Rect,
    label: String,
    label_dimensions: TextDimensions,
    sub_label: Option<String>,
    sub_label_dimensions: Option<TextDimensions>,
    is_selected: bool,
}

impl ToggleButton {
    fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}
