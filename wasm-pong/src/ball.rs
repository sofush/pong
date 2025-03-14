use std::time::Duration;

use crate::{
    bounding_box::BoundingBox, paddle::Paddle, screen::Screen, vec2::Vec2,
};
use web_sys::CanvasRenderingContext2d;

const SPEED: f64 = 0.5;
const SIZE: f64 = 0.05;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    bounding_box: BoundingBox,
    direction: Vec2,
}

impl Ball {
    fn calc_size(screen: Screen) -> Vec2 {
        let height = SIZE * screen.aspect_ratio();
        let width = SIZE;
        Vec2::new(width, height)
    }

    pub fn new(screen: Screen) -> Self {
        let size = Self::calc_size(screen);

        Self {
            bounding_box: BoundingBox {
                pos: Vec2::new(0.0, 0.0),
                size,
            },
            direction: Vec2::new(SPEED, SPEED),
        }
    }

    pub fn update(
        &mut self,
        screen: Screen,
        delta_time: Duration,
        right_paddle: Paddle,
        left_paddle: Paddle,
    ) {
        self.bounding_box.size = Self::calc_size(screen);
        self.bounding_box.pos.y +=
            self.direction.y * delta_time.as_secs_f64() * screen.aspect_ratio();
        self.bounding_box.pos.x += self.direction.x * delta_time.as_secs_f64();

        if self.bounding_box.pos.y >= 1.0 || self.bounding_box.pos.y <= -1.0 {
            self.direction.y = -self.direction.y;
            self.direction.y -= 0.01;
        }

        if self.bounding_box.collides_with(right_paddle.bounding_box()) {
            self.direction = Vec2::new(-SPEED, 0.0);
        }

        if self.bounding_box.collides_with(left_paddle.bounding_box()) {
            self.direction = Vec2::new(SPEED, 0.0);
        }

        if self.bounding_box.pos.x <= -1.0 {
            self.reset();
        }

        if self.bounding_box.pos.x >= 1.0 {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.bounding_box.pos = Vec2::new(0.0, 0.0);
    }

    pub fn draw(&self, screen: Screen, context: &CanvasRenderingContext2d) {
        let (x, y) = self.bounding_box.top_left().to_screen_coordinates(screen);
        let (width, height) = self.bounding_box.size.to_screen_size(screen);

        log::debug!("Drawing ball to x{} y{}, {}x{}", x, y, width, height);

        context
            .round_rect_with_f64(x, y, width, height, 1_000_000.0)
            .unwrap();
    }
}
