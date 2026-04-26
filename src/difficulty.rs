#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl Difficulty {
    pub fn label(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn block_type_count(&self) -> usize {
        match self {
            Difficulty::Easy => 4,
            Difficulty::Normal => 6,
            Difficulty::Hard => 8,
        }
    }
}
