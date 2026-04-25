use macroquad::{
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    math::Rect,
    text::TextDimensions,
};

use crate::{difficulty::Difficulty, grid_size::GridSize};

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
pub(super) enum ButtonStyle {
    Primary,
    Secondary,
    Toggle {
        is_selected: bool,
        sub_label: Option<String>,
        sub_label_dimensions: Option<TextDimensions>,
    },
}

pub(super) struct Button {
    pub(super) id: ButtonId,
    pub(super) bounds: Rect,
    pub(super) label: String,
    pub(super) label_dimensions: TextDimensions,
    pub(super) font_size: u16,
    pub(super) style: ButtonStyle,
}

impl Button {
    pub(super) fn new(
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

    pub(super) fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    pub(super) fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}
