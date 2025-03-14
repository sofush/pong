#[derive(Clone, Copy)]
pub struct Screen {
    /// Width of the screen in pixels.
    width: f64,
    /// Height of the screen in pixels.
    height: f64,
}

impl Screen {
    pub fn new(width: f64, height: f64) -> Screen {
        Self { width, height }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width / self.height
    }
}
