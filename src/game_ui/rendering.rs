use macroquad::{
    miniquad::window::set_mouse_cursor,
    shapes::draw_rectangle,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    constants::{
        style::{BACKGROUND_COLOR, BLOCK_INSET, GRID_BACKGROUND_COLOR},
        ui::{
            BODY_TEXT_SIZE, BUTTON_BACKGROUND_COLOR, BUTTON_SHADOW_COLOR, CARD_BORDER_COLOR,
            CONTAINER_INNER_PADDING, CORNER_RADIUS, LABEL_TEXT_COLOR, LABEL_TEXT_SIZE,
            LABEL_VALUE_GAP, LABEL_VALUE_SIZE, PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_HOVER_COLOR,
            PRIMARY_BUTTON_SHADOW_COLOR, TEXT_COLOR, TITLE_TEXT_SIZE, WINDOW_PADDING,
        },
    },
    drawing::draw_rounded_rect,
};

use super::buttons::{Button, ButtonStyle};

pub(super) fn render_status_panel(
    title_font: &Font,
    body_font: &Font,
    status_panel_height: f32,
    blocks_remaining: u32,
    score: u32,
) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let panel_y = screen_height - status_panel_height;

    draw_rectangle(
        0.0,
        panel_y,
        screen_width,
        status_panel_height,
        GRID_BACKGROUND_COLOR,
    );

    let card_y = panel_y + WINDOW_PADDING.y;
    let card_h = status_panel_height - WINDOW_PADDING.y * 2.0;

    let pause_btn_size = card_h;
    let pause_btn_x = screen_width - WINDOW_PADDING.x - pause_btn_size;
    let cards_end = pause_btn_x - WINDOW_PADDING.x;
    let card_w = (cards_end - WINDOW_PADDING.x - WINDOW_PADDING.x) / 2.0;

    let mut card_x = WINDOW_PADDING.x;
    card_x = render_datum_card(
        title_font,
        body_font,
        card_x,
        card_y,
        card_w,
        card_h,
        "Blocks left",
        &blocks_remaining.to_formatted_string(&Locale::en),
    );
    card_x += WINDOW_PADDING.x;
    render_datum_card(
        title_font,
        body_font,
        card_x,
        card_y,
        card_w,
        card_h,
        "Score",
        &score.to_formatted_string(&Locale::en),
    );
}

fn render_datum_card(
    title_font: &Font,
    body_font: &Font,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: &str,
    value: &str,
) -> f32 {
    let label_upper = label.to_uppercase();
    let label_dims = measure_text(&label_upper, Some(body_font), LABEL_TEXT_SIZE, 1.0);
    let value_dims = measure_text(value, Some(title_font), LABEL_VALUE_SIZE, 1.0);

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
            font: Some(body_font),
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
            font: Some(title_font),
            ..Default::default()
        },
    );

    x + w
}

pub(super) fn render_game_over(title_font: &Font, body_font: &Font, score: u32) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let text = "Game Over!";
    let dimensions = measure_text(text, Some(title_font), TITLE_TEXT_SIZE, 1.0);
    let y = (screen_height - dimensions.height) / 2.0;
    draw_text_ex(
        text,
        (screen_width - dimensions.width) / 2.0,
        y,
        TextParams {
            font_size: TITLE_TEXT_SIZE,
            color: TEXT_COLOR,
            font: Some(title_font),
            ..Default::default()
        },
    );

    let text = format!("Score: {}", score.to_formatted_string(&Locale::en));
    let dimensions = measure_text(&text, Some(body_font), BODY_TEXT_SIZE, 1.0);
    draw_text_ex(
        &text,
        (screen_width - dimensions.width) / 2.0,
        y + dimensions.height + 8.0,
        TextParams {
            font_size: BODY_TEXT_SIZE,
            color: TEXT_COLOR,
            font: Some(body_font),
            ..Default::default()
        },
    );
}

pub(super) fn render_main_menu(title_font: &Font) {
    let text = "Bleak Blocks";
    let dimensions = measure_text(text, Some(title_font), TITLE_TEXT_SIZE, 1.0);
    draw_text_ex(
        text,
        (screen_width() - dimensions.width) / 2.0,
        WINDOW_PADDING.y + dimensions.height,
        TextParams {
            font_size: TITLE_TEXT_SIZE,
            color: TEXT_COLOR,
            font: Some(title_font),
            ..Default::default()
        },
    );
}

pub(super) fn render_settings(
    title_font: &Font,
    body_font: &Font,
    grid_size_label_y: f32,
    difficulty_label_y: f32,
) {
    let text = "Settings";
    let dims = measure_text(text, Some(title_font), TITLE_TEXT_SIZE, 1.0);
    draw_text_ex(
        text,
        (screen_width() - dims.width) / 2.0,
        WINDOW_PADDING.y + dims.height,
        TextParams {
            font_size: TITLE_TEXT_SIZE,
            color: TEXT_COLOR,
            font: Some(title_font),
            ..Default::default()
        },
    );

    let label_dims = measure_text("A", Some(body_font), LABEL_TEXT_SIZE, 1.0);
    draw_text_ex(
        "GRID SIZE",
        WINDOW_PADDING.x,
        grid_size_label_y + label_dims.offset_y,
        TextParams {
            font_size: LABEL_TEXT_SIZE,
            color: LABEL_TEXT_COLOR,
            font: Some(body_font),
            ..Default::default()
        },
    );
    draw_text_ex(
        "DIFFICULTY",
        WINDOW_PADDING.x,
        difficulty_label_y + label_dims.offset_y,
        TextParams {
            font_size: LABEL_TEXT_SIZE,
            color: LABEL_TEXT_COLOR,
            font: Some(body_font),
            ..Default::default()
        },
    );
}

pub(super) fn render_button(title_font: &Font, button: &Button) {
    let (border_color, fill_color, hover_color, shadow_color) =
        if matches!(button.style, ButtonStyle::Primary) {
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

    draw_rounded_rect(
        button.bounds.x,
        button.bounds.y,
        button.bounds.w,
        button.bounds.h,
        CORNER_RADIUS,
        shadow_color,
    );
    draw_rounded_rect(
        button.bounds.x,
        button.bounds.y,
        button.bounds.w,
        face_h,
        CORNER_RADIUS,
        border_color,
    );
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
            font: Some(title_font),
            ..Default::default()
        },
    );
}

pub(super) fn render_toggle_button(title_font: &Font, body_font: &Font, button: &Button) {
    let ButtonStyle::Toggle {
        is_selected,
        sub_label,
        sub_label_dimensions,
    } = &button.style
    else {
        return;
    };

    let (border_color, fill_color, hover_color, shadow_color) = if *is_selected {
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

    draw_rounded_rect(
        button.bounds.x,
        button.bounds.y,
        button.bounds.w,
        button.bounds.h,
        CORNER_RADIUS,
        shadow_color,
    );
    draw_rounded_rect(
        button.bounds.x,
        button.bounds.y,
        button.bounds.w,
        face_h,
        CORNER_RADIUS,
        border_color,
    );
    draw_rounded_rect(
        button.bounds.x + 1.0,
        button.bounds.y + 1.0,
        button.bounds.w - 2.0,
        face_h - 2.0,
        CORNER_RADIUS - 1.0,
        fill,
    );

    let center_x = button.bounds.center().x;

    if let (Some(sub_label), Some(sub_dims)) = (sub_label, sub_label_dimensions) {
        let total_text_h = button.label_dimensions.height + 4.0 + sub_dims.height;
        let text_top = button.bounds.y + (face_h - total_text_h) / 2.0;

        draw_text_ex(
            &button.label,
            center_x - button.label_dimensions.width / 2.0,
            text_top + button.label_dimensions.offset_y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(title_font),
                ..Default::default()
            },
        );
        draw_text_ex(
            sub_label,
            center_x - sub_dims.width / 2.0,
            text_top + button.label_dimensions.height + 4.0 + sub_dims.offset_y,
            TextParams {
                font_size: LABEL_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(body_font),
                ..Default::default()
            },
        );
    } else {
        let face_center_y = button.bounds.y + face_h / 2.0;
        draw_text_ex(
            &button.label,
            center_x - button.label_dimensions.width / 2.0,
            face_center_y - button.label_dimensions.height / 2.0 + button.label_dimensions.offset_y,
            TextParams {
                font_size: BODY_TEXT_SIZE,
                color: TEXT_COLOR,
                font: Some(title_font),
                ..Default::default()
            },
        );
    }
}
