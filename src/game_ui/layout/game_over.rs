use macroquad::{
    math::Rect,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{screen_height, screen_width},
};
use num_format::{Locale, ToFormattedString};

use crate::constants::{
    style::BACKGROUND_COLOR,
    ui::{
        BODY_TEXT_SIZE, BUTTON_PADDING, CARD_BORDER_COLOR, CORNER_RADIUS, MODAL_PADDING,
        MODAL_SHADOW_COLOR, TEXT_COLOR, TITLE_TEXT_SIZE,
    },
};
use crate::drawing::draw_rounded_rect;
use crate::platform::{scale, scale_font, top_inset};

use super::super::Fonts;
use super::super::buttons::{Button, ButtonId, ButtonStyle};
use super::compute_button_stack;

pub struct GameOverLayout {
    pub buttons: Vec<Button>,
    pub modal_rect: Rect,
}

impl GameOverLayout {
    pub fn compute(title_font: &Font, body_font: &Font) -> Self {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let modal_padding = scale(MODAL_PADDING);
        let button_padding_y = scale(BUTTON_PADDING.y);
        let title_font_size = scale_font(TITLE_TEXT_SIZE);
        let body_font_size = scale_font(BODY_TEXT_SIZE);

        let title_dims = measure_text("Game Over!", Some(title_font), title_font_size, 1.0);
        let score_dims = measure_text("Score: 000,000", Some(body_font), body_font_size, 1.0);
        let btn_label_dims = measure_text("Menu", Some(title_font), body_font_size, 1.0);
        let btn_h = btn_label_dims.height + 2.0 * button_padding_y + scale(2.0); // face_h + BLOCK_INSET

        let content_h = title_dims.height
            + scale(12.0)
            + score_dims.height
            + scale(20.0)
            + btn_h
            + modal_padding;
        let modal_h = content_h + modal_padding * 2.0;
        let modal_w = (screen_w * 0.6).max(scale(280.0)).min(scale(400.0));
        let modal_x = (screen_w - modal_w) / 2.0;
        let top = top_inset();
        let modal_y = top + (screen_h - top - modal_h) / 2.0;

        let btn_anchor_y = modal_y + modal_h - modal_padding;
        let buttons = compute_button_stack(
            title_font,
            &[("Menu", ButtonId::Menu, ButtonStyle::Secondary)],
            btn_anchor_y,
        );

        Self {
            buttons,
            modal_rect: Rect::new(modal_x, modal_y, modal_w, modal_h),
        }
    }

    pub fn render(&self, fonts: Fonts, score: u32) {
        let r = &self.modal_rect;
        let corner = scale(CORNER_RADIUS) * 2.0;
        let modal_padding = scale(MODAL_PADDING);
        let title_font_size = scale_font(TITLE_TEXT_SIZE);
        let body_font_size = scale_font(BODY_TEXT_SIZE);

        // Drop shadow
        draw_rounded_rect(r.x + 2.0, r.y + 6.0, r.w, r.h, corner, MODAL_SHADOW_COLOR);

        // Card background + border
        draw_rounded_rect(r.x, r.y, r.w, r.h, corner, CARD_BORDER_COLOR);
        draw_rounded_rect(
            r.x + 1.0,
            r.y + 1.0,
            r.w - 2.0,
            r.h - 2.0,
            corner - 1.0,
            BACKGROUND_COLOR,
        );

        let cx = r.x + r.w / 2.0;
        let mut y = r.y + modal_padding;

        // Title
        let title = "Game Over!";
        let title_dims = measure_text(title, Some(fonts.title), title_font_size, 1.0);
        draw_text_ex(
            title,
            cx - title_dims.width / 2.0,
            y + title_dims.offset_y,
            TextParams {
                font_size: title_font_size,
                color: TEXT_COLOR,
                font: Some(fonts.title),
                ..Default::default()
            },
        );
        y += title_dims.height + scale(12.0);

        // Score
        let score_text = format!("Score: {}", score.to_formatted_string(&Locale::en));
        let score_dims = measure_text(&score_text, Some(fonts.body), body_font_size, 1.0);
        draw_text_ex(
            &score_text,
            cx - score_dims.width / 2.0,
            y + score_dims.offset_y,
            TextParams {
                font_size: body_font_size,
                color: TEXT_COLOR,
                font: Some(fonts.body),
                ..Default::default()
            },
        );
    }
}
