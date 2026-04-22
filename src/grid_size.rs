#[derive(Copy, Clone, PartialEq, Default)]
pub enum GridSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl GridSize {
    pub fn label(&self) -> &str {
        match self {
            GridSize::Small => "Small",
            GridSize::Medium => "Medium",
            GridSize::Large => "Large",
            GridSize::ExtraLarge => "X-Large",
        }
    }

    pub fn size_hint(&self, is_landscape: bool) -> String {
        let (rows, cols) = self.grid_dims(is_landscape);
        format!("{rows}×{cols}")
    }

    /// Returns (rows, cols). Portrait = more rows, landscape = more cols.
    pub fn grid_dims(&self, is_landscape: bool) -> (u32, u32) {
        let (portrait_rows, portrait_cols) = match self {
            GridSize::Small => (8, 6),
            GridSize::Medium => (13, 10),
            GridSize::Large => (18, 14),
            GridSize::ExtraLarge => (24, 18),
        };
        if is_landscape {
            (portrait_cols, portrait_rows)
        } else {
            (portrait_rows, portrait_cols)
        }
    }
}
