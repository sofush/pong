#![allow(dead_code)]

use crate::vec2::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    /// Position (X and Y coordinates) of the object.
    pub pos: Vec2,
    /// Width and height of the object.
    pub size: Vec2,
}

impl BoundingBox {
    pub fn top(&self) -> f64 {
        let half_height = self.size.y / 2.0;
        self.pos.y - half_height
    }

    pub fn bottom(&self) -> f64 {
        let half_height = self.size.y / 2.0;
        self.pos.y + half_height
    }

    pub fn left(&self) -> f64 {
        let half_width = self.size.x / 2.0;
        self.pos.x - half_width
    }

    pub fn right(&self) -> f64 {
        let half_width = self.size.x / 2.0;
        self.pos.x + half_width
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.right(), self.bottom())
    }

    pub fn collides_with(&self, other: BoundingBox) -> bool {
        self.left() < other.left() + other.size.x
            && self.left() + self.size.x > other.left()
            && self.top() < other.top() + other.size.y
            && self.top() + self.size.y > other.top()
    }
}
