use macroquad::{
    math::Rect,
    shapes::draw_rectangle,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::{
    constants::{
        style::{BACKGROUND_COLOR, BLOCK_INSET, GRID_BACKGROUND_COLOR},
        ui::{
            BODY_TEXT_SIZE, CARD_BORDER_COLOR, CONTAINER_INNER_PADDING, CORNER_RADIUS,
            LABEL_TEXT_COLOR, LABEL_TEXT_SIZE, LABEL_VALUE_GAP, LABEL_VALUE_SIZE, PAUSE_ICON_SIZE,
            TEXT_COLOR, WINDOW_PADDING,
        },
    },
    drawing::draw_rounded_rect,
};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};

pub struct PlayingLayout {
    pub status_panel_height: f32,
    pub buttons: Vec<Button>,
}

impl PlayingLayout {
    pub fn compute(title_font: &Font, body_font: &Font) -> Self {
        let label_dims = measure_text("A", Some(body_font), BODY_TEXT_SIZE, 1.0);
        let value_dims = measure_text("A", Some(title_font), LABEL_VALUE_SIZE, 1.0);
        let status_panel_height = WINDOW_PADDING.y * 2.0
            + CONTAINER_INNER_PADDING * 2.0
            + label_dims.height
            + LABEL_VALUE_GAP
            + value_dims.height;

        let panel_y = screen_height() - status_panel_height;
        let card_h = status_panel_height - WINDOW_PADDING.y * 2.0;
        let pause_label = "||";
        let pause_dims = measure_text(pause_label, Some(title_font), PAUSE_ICON_SIZE, 1.0);
        let btn_w = card_h;
        let btn_h = card_h + BLOCK_INSET;
        let btn_x = screen_width() - WINDOW_PADDING.x - btn_w;
        let btn_y = panel_y + WINDOW_PADDING.y;

        Self {
            status_panel_height,
            buttons: vec![Button::new(
                ButtonId::Pause,
                Rect::new(btn_x, btn_y, btn_w, btn_h),
                pause_label.to_string(),
                pause_dims,
                PAUSE_ICON_SIZE,
                ButtonStyle::Secondary,
            )],
        }
    }

    pub fn render(&self, fonts: Fonts, blocks_remaining: u32, score: u32) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let panel_y = screen_h - self.status_panel_height;

        draw_rectangle(
            0.0,
            panel_y,
            screen_w,
            self.status_panel_height,
            GRID_BACKGROUND_COLOR,
        );

        let card_y = panel_y + WINDOW_PADDING.y;
        let card_h = self.status_panel_height - WINDOW_PADDING.y * 2.0;
        let pause_btn_size = card_h;
        let pause_btn_x = screen_w - WINDOW_PADDING.x - pause_btn_size;
        let cards_end = pause_btn_x - WINDOW_PADDING.x;
        let card_w = (cards_end - WINDOW_PADDING.x - WINDOW_PADDING.x) / 2.0;

        let mut card_x = WINDOW_PADDING.x;
        card_x = render_datum_card(
            fonts.title,
            fonts.body,
            card_x,
            card_y,
            card_w,
            card_h,
            "Blocks left",
            &blocks_remaining.to_formatted_string(&Locale::en),
        );
        card_x += WINDOW_PADDING.x;
        render_datum_card(
            fonts.title,
            fonts.body,
            card_x,
            card_y,
            card_w,
            card_h,
            "Score",
            &score.to_formatted_string(&Locale::en),
        );
    }
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
