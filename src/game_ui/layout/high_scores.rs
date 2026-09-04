use macroquad::{
    math::Rect,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::screen_width,
};
use num_format::{Locale, ToFormattedString};

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
    high_scores::HighScores,
    platform::{scale, scale_font, top_inset},
};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};
use super::compute_button_stack;

struct GridSizeSection {
    label: String,
    label_y: f32,
    scores_start_y: f32,
    entries: Vec<u32>,
}

pub struct HighScoresLayout {
    pub buttons: Vec<Button>,
    pub difficulty_label_y: f32,
    pub score_row_height: f32,
    sections: Vec<GridSizeSection>,
}

impl HighScoresLayout {
    pub fn compute(
        title_font: &Font,
        body_font: &Font,
        difficulty: Difficulty,
        high_scores: &HighScores,
    ) -> Self {
        let window_padding_x = scale(WINDOW_PADDING.x);
        let button_padding_y = scale(BUTTON_PADDING.y);
        let block_inset = scale(BLOCK_INSET);
        let body_font_size = scale_font(BODY_TEXT_SIZE);
        let label_font_size = scale_font(LABEL_TEXT_SIZE);

        let available_w = screen_width() - 2.0 * window_padding_x;
        let btn_gap = window_padding_x;

        let title_dims = measure_text(
            "High Scores",
            Some(title_font),
            scale_font(TITLE_TEXT_SIZE),
            1.0,
        );
        let mut current_y = top_inset() + scale(WINDOW_PADDING.y) + title_dims.height + scale(16.0);

        // Difficulty filter row
        let label_a_dims = measure_text("A", Some(body_font), label_font_size, 1.0);
        let difficulty_label_y = current_y;
        current_y += label_a_dims.height + scale(8.0);

        let diff_btn_w = (available_w - 2.0 * btn_gap) / 3.0;
        let diff_main_dims = measure_text("Normal", Some(title_font), body_font_size, 1.0);
        let diff_face_h = button_padding_y + diff_main_dims.height + button_padding_y;
        let diff_btn_h = diff_face_h + block_inset;

        let mut buttons = Vec::new();
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

        // One section per grid size
        let score_row_dims = measure_text("A", Some(body_font), body_font_size, 1.0);
        let score_row_height = score_row_dims.height + scale(6.0);

        let gs_variants = [
            GridSize::Small,
            GridSize::Medium,
            GridSize::Large,
            GridSize::ExtraLarge,
        ];
        let mut sections = Vec::new();
        for gs in gs_variants {
            current_y += scale(20.0);
            let label_y = current_y;
            current_y += label_a_dims.height + scale(8.0);
            let scores_start_y = current_y;

            let entries: Vec<u32> = high_scores
                .get_scores_for(gs, difficulty)
                .iter()
                .map(|e| e.score)
                .collect();

            current_y += entries.len().max(1) as f32 * score_row_height;

            sections.push(GridSizeSection {
                label: gs.label().to_uppercase(),
                label_y,
                scores_start_y,
                entries,
            });
        }

        // Back button
        current_y += scale(24.0);
        let back_dims = measure_text("Back", Some(title_font), body_font_size, 1.0);
        let back_baseline = current_y + back_dims.offset_y + button_padding_y;
        buttons.extend(compute_button_stack(
            title_font,
            &[("Back", ButtonId::Back, ButtonStyle::Secondary)],
            back_baseline,
        ));

        Self {
            buttons,
            difficulty_label_y,
            score_row_height,
            sections,
        }
    }

    pub fn render(&self, fonts: Fonts) {
        let window_padding_x = scale(WINDOW_PADDING.x);
        let body_font_size = scale_font(BODY_TEXT_SIZE);
        let label_font_size = scale_font(LABEL_TEXT_SIZE);

        let text = "High Scores";
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

        let label_a_dims = measure_text("A", Some(fonts.body), label_font_size, 1.0);

        draw_text_ex(
            "DIFFICULTY",
            window_padding_x,
            self.difficulty_label_y + label_a_dims.offset_y,
            TextParams {
                font_size: label_font_size,
                color: LABEL_TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );

        let score_a_dims = measure_text("A", Some(fonts.body), body_font_size, 1.0);

        for section in &self.sections {
            draw_text_ex(
                &section.label,
                window_padding_x,
                section.label_y + label_a_dims.offset_y,
                TextParams {
                    font_size: label_font_size,
                    color: LABEL_TEXT_COLOR,
                    font: Some(fonts.body),
                    ..Default::default()
                },
            );

            if section.entries.is_empty() {
                draw_text_ex(
                    "No scores yet",
                    window_padding_x,
                    section.scores_start_y + score_a_dims.offset_y,
                    TextParams {
                        font_size: body_font_size,
                        color: LABEL_TEXT_COLOR,
                        font: Some(fonts.body),
                        ..Default::default()
                    },
                );
            } else {
                for (i, score) in section.entries.iter().enumerate() {
                    let row_text = format!("#{} {}", i + 1, score.to_formatted_string(&Locale::en));
                    let y = section.scores_start_y
                        + i as f32 * self.score_row_height
                        + score_a_dims.offset_y;
                    draw_text_ex(
                        &row_text,
                        window_padding_x,
                        y,
                        TextParams {
                            font_size: body_font_size,
                            color: TEXT_COLOR,
                            font: Some(fonts.body),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
