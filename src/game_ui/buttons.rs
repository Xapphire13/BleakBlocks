use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Rect,
    miniquad::window::set_mouse_cursor,
    text::{TextDimensions, TextParams, draw_text_ex},
};

use crate::{
    constants::{
        style::BLOCK_INSET,
        ui::{
            BUTTON_BACKGROUND_COLOR, BUTTON_SHADOW_COLOR, CARD_BORDER_COLOR, CORNER_RADIUS,
            LABEL_TEXT_SIZE, PRIMARY_BUTTON_COLOR, PRIMARY_BUTTON_HOVER_COLOR,
            PRIMARY_BUTTON_SHADOW_COLOR, TEXT_COLOR,
        },
    },
    difficulty::Difficulty,
    drawing::draw_rounded_rect,
    grid_size::GridSize,
};

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

#[derive(Clone)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Toggle {
        is_selected: bool,
        sub_label: Option<String>,
        sub_label_dimensions: Option<TextDimensions>,
    },
}

pub struct Button {
    pub id: ButtonId,
    pub bounds: Rect,
    pub label: String,
    pub label_dimensions: TextDimensions,
    pub font_size: u16,
    pub style: ButtonStyle,
}

impl Button {
    pub fn new(
        id: ButtonId,
        bounds: Rect,
        label: String,
        label_dimensions: TextDimensions,
        font_size: u16,
        style: ButtonStyle,
    ) -> Self {
        Self {
            id,
            bounds,
            label,
            label_dimensions,
            font_size,
            style,
        }
    }

    pub fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    pub fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }

    pub fn render(&self, fonts: super::Fonts) {
        let (border_color, fill_color, hover_color, shadow_color) = match &self.style {
            ButtonStyle::Primary => (
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_HOVER_COLOR,
                PRIMARY_BUTTON_SHADOW_COLOR,
            ),
            ButtonStyle::Toggle {
                is_selected: true, ..
            } => (
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_COLOR,
                PRIMARY_BUTTON_HOVER_COLOR,
                PRIMARY_BUTTON_SHADOW_COLOR,
            ),
            _ => (
                CARD_BORDER_COLOR,
                BUTTON_BACKGROUND_COLOR,
                CARD_BORDER_COLOR,
                BUTTON_SHADOW_COLOR,
            ),
        };

        let fill = if self.is_hovered() {
            set_mouse_cursor(macroquad::miniquad::CursorIcon::Pointer);
            hover_color
        } else {
            fill_color
        };

        let face_h = self.bounds.h - BLOCK_INSET;

        draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            CORNER_RADIUS,
            shadow_color,
        );
        draw_rounded_rect(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            face_h,
            CORNER_RADIUS,
            border_color,
        );
        draw_rounded_rect(
            self.bounds.x + 1.0,
            self.bounds.y + 1.0,
            self.bounds.w - 2.0,
            face_h - 2.0,
            CORNER_RADIUS - 1.0,
            fill,
        );

        let center_x = self.bounds.center().x;

        if let ButtonStyle::Toggle {
            sub_label: Some(sub_label),
            sub_label_dimensions: Some(sub_dims),
            ..
        } = &self.style
        {
            let total_text_h = self.label_dimensions.height + 4.0 + sub_dims.height;
            let text_top = self.bounds.y + (face_h - total_text_h) / 2.0;
            draw_text_ex(
                &self.label,
                center_x - self.label_dimensions.width / 2.0,
                text_top + self.label_dimensions.offset_y,
                TextParams {
                    font_size: self.font_size,
                    color: TEXT_COLOR,
                    font: Some(fonts.title),
                    ..Default::default()
                },
            );
            draw_text_ex(
                sub_label,
                center_x - sub_dims.width / 2.0,
                text_top + self.label_dimensions.height + 4.0 + sub_dims.offset_y,
                TextParams {
                    font_size: LABEL_TEXT_SIZE,
                    color: TEXT_COLOR,
                    font: Some(fonts.body),
                    ..Default::default()
                },
            );
        } else {
            let face_center_y = self.bounds.y + face_h / 2.0;
            draw_text_ex(
                &self.label,
                center_x - self.label_dimensions.width / 2.0,
                face_center_y - self.label_dimensions.height / 2.0 + self.label_dimensions.offset_y,
                TextParams {
                    font_size: self.font_size,
                    color: TEXT_COLOR,
                    font: Some(fonts.title),
                    ..Default::default()
                },
            );
        }
    }
}
