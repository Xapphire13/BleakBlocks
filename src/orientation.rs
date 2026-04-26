#[derive(Copy, Clone, PartialEq, Default)]
pub enum Orientation {
    Portrait,
    #[default]
    Landscape,
}

impl Orientation {
    pub fn label(self) -> &'static str {
        match self {
            Orientation::Portrait => "Portrait",
            Orientation::Landscape => "Landscape",
        }
    }
}
