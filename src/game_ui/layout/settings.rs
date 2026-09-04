use macroquad::{
    math::Rect,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::screen_width,
};

use crate::{
    constants::{
        style::BLOCK_INSET,
        ui::{
            BODY_TEXT_SIZE, BUTTON_PADDING, LABEL_TEXT_COLOR, LABEL_TEXT_SIZE, TEXT_COLOR,
            TITLE_TEXT_SIZE, WINDOW_PADDING,
        },
    },
    difficulty::Difficulty,
    grid_size::GridSize,
    orientation::Orientation,
    platform::{scale, scale_font, top_inset},
};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};
use super::compute_button_stack;

pub struct SettingsLayout {
    pub grid_size_label_y: f32,
    pub orientation_label_y: f32,
    pub difficulty_label_y: f32,
    pub buttons: Vec<Button>,
}

impl SettingsLayout {
    pub fn compute(
        title_font: &Font,
        body_font: &Font,
        grid_size: GridSize,
        difficulty: Difficulty,
        orientation: Orientation,
    ) -> Self {
        let window_padding_x = scale(WINDOW_PADDING.x);
        let button_padding_y = scale(BUTTON_PADDING.y);
        let block_inset = scale(BLOCK_INSET);
        let body_font_size = scale_font(BODY_TEXT_SIZE);
        let label_font_size = scale_font(LABEL_TEXT_SIZE);

        let available_w = screen_width() - 2.0 * window_padding_x;
        let btn_gap = window_padding_x;

        let title_dims = measure_text(
            "Settings",
            Some(title_font),
            scale_font(TITLE_TEXT_SIZE),
            1.0,
        );
        let mut current_y = top_inset() + scale(WINDOW_PADDING.y) + title_dims.height + scale(16.0);

        let gs_label_dims = measure_text("A", Some(body_font), label_font_size, 1.0);
        let grid_size_label_y = current_y;
        current_y += gs_label_dims.height + scale(8.0);

        let gs_btn_w = (available_w - btn_gap) / 2.0;
        let gs_main_dims = measure_text("X-Large", Some(title_font), body_font_size, 1.0);
        let gs_sub_dims = measure_text("00×00", Some(body_font), label_font_size, 1.0);
        let label_gap = scale(4.0);
        let gs_face_h = button_padding_y
            + gs_main_dims.height
            + label_gap
            + gs_sub_dims.height
            + button_padding_y;
        let gs_btn_h = gs_face_h + block_inset;

        let gs_variants = [
            GridSize::Small,
            GridSize::Medium,
            GridSize::Large,
            GridSize::ExtraLarge,
        ];
        let mut buttons = Vec::new();
        for (i, gs) in gs_variants.iter().enumerate() {
            let row = i / 2;
            let col = i % 2;
            let x = window_padding_x + col as f32 * (gs_btn_w + btn_gap);
            let y = current_y + row as f32 * (gs_btn_h + btn_gap);
            let label = gs.label().to_string();
            let sub_label = gs.size_hint(orientation);
            let label_dims = measure_text(&label, Some(title_font), body_font_size, 1.0);
            let sub_label_dims = measure_text(&sub_label, Some(body_font), label_font_size, 1.0);
            buttons.push(Button::new(
                ButtonId::SetGridSize(*gs),
                Rect::new(x, y, gs_btn_w, gs_btn_h),
                label,
                label_dims,
                body_font_size,
                ButtonStyle::Toggle {
                    is_selected: *gs == grid_size,
                    sub_label: Some(sub_label),
                    sub_label_dimensions: Some(sub_label_dims),
                },
            ));
        }
        current_y += 2.0 * gs_btn_h + btn_gap;

        current_y += scale(20.0);
        let orient_label_dims = measure_text("A", Some(body_font), label_font_size, 1.0);
        let orientation_label_y = current_y;
        current_y += orient_label_dims.height + scale(8.0);

        let orient_btn_w = (available_w - btn_gap) / 2.0;
        let orient_main_dims = measure_text("Landscape", Some(title_font), body_font_size, 1.0);
        let orient_face_h = button_padding_y + orient_main_dims.height + button_padding_y;
        let orient_btn_h = orient_face_h + block_inset;

        let orient_variants = [Orientation::Portrait, Orientation::Landscape];
        for (i, o) in orient_variants.iter().enumerate() {
            let x = window_padding_x + i as f32 * (orient_btn_w + btn_gap);
            let label = o.label().to_string();
            let label_dims = measure_text(&label, Some(title_font), body_font_size, 1.0);
            buttons.push(Button::new(
                ButtonId::SetOrientation(*o),
                Rect::new(x, current_y, orient_btn_w, orient_btn_h),
                label,
                label_dims,
                body_font_size,
                ButtonStyle::Toggle {
                    is_selected: *o == orientation,
                    sub_label: None,
                    sub_label_dimensions: None,
                },
            ));
        }
        current_y += orient_btn_h;

        current_y += scale(20.0);
        let diff_label_dims = measure_text("A", Some(body_font), label_font_size, 1.0);
        let difficulty_label_y = current_y;
        current_y += diff_label_dims.height + scale(8.0);

        let diff_btn_w = (available_w - 2.0 * btn_gap) / 3.0;
        let diff_main_dims = measure_text("Normal", Some(title_font), body_font_size, 1.0);
        let diff_face_h = button_padding_y + diff_main_dims.height + button_padding_y;
        let diff_btn_h = diff_face_h + block_inset;

        let diff_variants = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
        for (i, diff) in diff_variants.iter().enumerate() {
            let x = window_padding_x + i as f32 * (diff_btn_w + btn_gap);
            let label = diff.label().to_string();
            let label_dims = measure_text(&label, Some(title_font), body_font_size, 1.0);
            buttons.push(Button::new(
                ButtonId::SetDifficulty(*diff),
                Rect::new(x, current_y, diff_btn_w, diff_btn_h),
                label,
                label_dims,
                body_font_size,
                ButtonStyle::Toggle {
                    is_selected: *diff == difficulty,
                    sub_label: None,
                    sub_label_dimensions: None,
                },
            ));
        }
        current_y += diff_btn_h;

        current_y += scale(24.0);
        let back_dims = measure_text("Back", Some(title_font), body_font_size, 1.0);
        let back_baseline = current_y + back_dims.offset_y + button_padding_y;
        buttons.extend(compute_button_stack(
            title_font,
            &[("Back", ButtonId::Back, ButtonStyle::Secondary)],
            back_baseline,
        ));

        Self {
            grid_size_label_y,
            orientation_label_y,
            difficulty_label_y,
            buttons,
        }
    }

    pub fn render(&self, fonts: Fonts) {
        let window_padding_x = scale(WINDOW_PADDING.x);
        let label_font_size = scale_font(LABEL_TEXT_SIZE);

        let text = "Settings";
        let title_font_size = scale_font(TITLE_TEXT_SIZE);
        let dims = measure_text(text, Some(fonts.title), title_font_size, 1.0);
        draw_text_ex(
            text,
            (screen_width() - dims.width) / 2.0,
            top_inset() + scale(WINDOW_PADDING.y) + dims.height,
            TextParams {
                font_size: title_font_size,
                color: TEXT_COLOR,
                font: Some(fonts.title),
                ..Default::default()
            },
        );

        let label_dims = measure_text("A", Some(fonts.body), label_font_size, 1.0);
        draw_text_ex(
            "GRID SIZE",
            window_padding_x,
            self.grid_size_label_y + label_dims.offset_y,
            TextParams {
                font_size: label_font_size,
                color: LABEL_TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );
        draw_text_ex(
            "ORIENTATION",
            window_padding_x,
            self.orientation_label_y + label_dims.offset_y,
            TextParams {
                font_size: label_font_size,
                color: LABEL_TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );
        draw_text_ex(
            "DIFFICULTY",
            window_padding_x,
            self.difficulty_label_y + label_dims.offset_y,
            TextParams {
                font_size: label_font_size,
                color: LABEL_TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );
    }
}
