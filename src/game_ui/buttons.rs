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

pub(super) struct Button {
    pub(super) id: ButtonId,
    pub(super) bounds: Rect,
    pub(super) label: String,
    pub(super) label_dimensions: TextDimensions,
    pub(super) font_size: u16,
    pub(super) is_primary: bool,
}

impl Button {
    pub(super) fn new(
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

    pub(super) fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    pub(super) fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}

pub(super) struct ToggleButton {
    pub(super) id: ButtonId,
    pub(super) bounds: Rect,
    pub(super) label: String,
    pub(super) label_dimensions: TextDimensions,
    pub(super) sub_label: Option<String>,
    pub(super) sub_label_dimensions: Option<TextDimensions>,
    pub(super) is_selected: bool,
}

impl ToggleButton {
    pub(super) fn is_hovered(&self) -> bool {
        self.bounds.contains(mouse_position().into())
    }

    pub(super) fn is_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left) && self.is_hovered()
    }
}
