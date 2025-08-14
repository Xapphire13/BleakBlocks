pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Bounds {
    pub fn is_within(&self, x: f32, y: f32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

pub trait HasBounds {
    fn get_bounds(&self) -> Bounds;

    fn is_within_bounds(&self, x: f32, y: f32) -> bool {
        self.get_bounds().is_within(x, y)
    }
}
