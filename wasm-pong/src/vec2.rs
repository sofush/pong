use crate::screen::Screen;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2 {
    /// X-coordinate of the center in normalized device coordinates.
    pub x: f64,
    /// Y-coordinate of the center in normalized device coordinates.
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Self { x, y }
    }

    pub fn to_screen_coordinates(&self, screen: Screen) -> (f64, f64) {
        let half_width = screen.width() / 2.0;
        let half_height = screen.height() / 2.0;
        let x = half_width + (half_width * self.x);
        let y = half_height + (half_height * self.y);
        (x, y)
    }

    pub fn to_screen_size(&self, screen: Screen) -> (f64, f64) {
        let ndc_width = self.x / 2.0;
        let ndc_height = self.y / 2.0;
        let width = screen.width() * ndc_width;
        let height = screen.height() * ndc_height;
        (width, height)
    }
}
