use macroquad::{
    math::Rect,
    text::{Font, TextDimensions, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    constants::{
        style::BLOCK_INSET,
        ui::{BODY_TEXT_SIZE, BUTTON_PADDING, LABEL_TEXT_SIZE, TITLE_TEXT_SIZE, WINDOW_PADDING},
    },
    difficulty::Difficulty,
    grid_size::GridSize,
};

use super::buttons::{Button, ButtonId, ButtonStyle};

pub(super) fn compute_settings_layout(
    title_font: &Font,
    body_font: &Font,
    grid_size: GridSize,
    difficulty: Difficulty,
) -> (Vec<Button>, f32, f32) {
    let is_landscape = screen_width() > screen_height();
    let available_w = screen_width() - 2.0 * WINDOW_PADDING.x;
    let btn_gap = WINDOW_PADDING.x;

    let title_dims = measure_text("Settings", Some(title_font), TITLE_TEXT_SIZE, 1.0);
    let mut current_y = WINDOW_PADDING.y + title_dims.height + 16.0;

    // Grid size toggles (2×2 layout)
    let gs_label_dims = measure_text("A", Some(body_font), LABEL_TEXT_SIZE, 1.0);
    let gs_label_y = current_y;
    current_y += gs_label_dims.height + 8.0;

    let gs_btn_w = (available_w - btn_gap) / 2.0;
    let gs_main_dims = measure_text("X-Large", Some(title_font), BODY_TEXT_SIZE, 1.0);
    let gs_sub_dims = measure_text("00×00", Some(body_font), LABEL_TEXT_SIZE, 1.0);
    let gs_face_h =
        BUTTON_PADDING.y + gs_main_dims.height + 4.0 + gs_sub_dims.height + BUTTON_PADDING.y;
    let gs_btn_h = gs_face_h + BLOCK_INSET;

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
        let x = WINDOW_PADDING.x + col as f32 * (gs_btn_w + btn_gap);
        let y = current_y + row as f32 * (gs_btn_h + btn_gap);
        let label = gs.label().to_string();
        let sub_label = gs.size_hint(is_landscape);
        let label_dims = measure_text(&label, Some(title_font), BODY_TEXT_SIZE, 1.0);
        let sub_label_dims = measure_text(&sub_label, Some(body_font), LABEL_TEXT_SIZE, 1.0);
        buttons.push(Button::new(
            ButtonId::SetGridSize(*gs),
            Rect::new(x, y, gs_btn_w, gs_btn_h),
            label,
            label_dims,
            BODY_TEXT_SIZE,
            ButtonStyle::Toggle {
                is_selected: *gs == grid_size,
                sub_label: Some(sub_label),
                sub_label_dimensions: Some(sub_label_dims),
            },
        ));
    }
    current_y += 2.0 * gs_btn_h + btn_gap;

    // Difficulty toggles (1×3 layout)
    current_y += 20.0;
    let diff_label_dims = measure_text("A", Some(body_font), LABEL_TEXT_SIZE, 1.0);
    let diff_label_y = current_y;
    current_y += diff_label_dims.height + 8.0;

    let diff_btn_w = (available_w - 2.0 * btn_gap) / 3.0;
    let diff_main_dims = measure_text("Normal", Some(title_font), BODY_TEXT_SIZE, 1.0);
    let diff_face_h = BUTTON_PADDING.y + diff_main_dims.height + BUTTON_PADDING.y;
    let diff_btn_h = diff_face_h + BLOCK_INSET;

    let diff_variants = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
    for (i, diff) in diff_variants.iter().enumerate() {
        let x = WINDOW_PADDING.x + i as f32 * (diff_btn_w + btn_gap);
        let label = diff.label().to_string();
        let label_dims = measure_text(&label, Some(title_font), BODY_TEXT_SIZE, 1.0);
        buttons.push(Button::new(
            ButtonId::SetDifficulty(*diff),
            Rect::new(x, current_y, diff_btn_w, diff_btn_h),
            label,
            label_dims,
            BODY_TEXT_SIZE,
            ButtonStyle::Toggle {
                is_selected: *diff == difficulty,
                sub_label: None,
                sub_label_dimensions: None,
            },
        ));
    }
    current_y += diff_btn_h;

    // Back button
    current_y += 24.0;
    let back_dims = measure_text("Back", Some(title_font), BODY_TEXT_SIZE, 1.0);
    let back_baseline = current_y + back_dims.offset_y + BUTTON_PADDING.y;
    buttons.extend(compute_button_stack(
        title_font,
        &[("Back", ButtonId::Back, ButtonStyle::Secondary)],
        back_baseline,
    ));

    (buttons, gs_label_y, diff_label_y)
}

pub(super) fn compute_button_stack(
    title_font: &Font,
    items: &[(&str, ButtonId, ButtonStyle)],
    start_y: f32,
) -> Vec<Button> {
    let measurements: Vec<TextDimensions> = items
        .iter()
        .map(|(text, _, _)| measure_text(text, Some(title_font), BODY_TEXT_SIZE, 1.0))
        .collect();

    let max_btn_w = measurements
        .iter()
        .map(|d| d.width + 2.0 * BUTTON_PADDING.x)
        .fold(0.0f32, f32::max);

    let center_x = screen_width() / 2.0;
    let mut y = start_y;
    let mut buttons = Vec::with_capacity(items.len());

    for (item, dims) in items.iter().zip(measurements.iter()) {
        let (text, id, style): &(&str, ButtonId, ButtonStyle) = item;
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
            style.clone(),
        ));
        y += face_h + BLOCK_INSET + 8.0;
    }

    buttons
}
