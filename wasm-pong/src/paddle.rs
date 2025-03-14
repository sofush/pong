use crate::{
    ball::Ball, bounding_box::BoundingBox, screen::Screen, vec2::Vec2,
};
use web_sys::CanvasRenderingContext2d;

const HEIGHT: f64 = 0.45;
const WIDTH: f64 = 0.035;
const MARGIN: f64 = 0.05;
const SPEED: f64 = 0.85;

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Down,
    Up,
    None,
}

#[derive(Clone, Copy)]
pub struct Paddle {
    bounding_box: BoundingBox,
    side: Side,
    direction: Direction,
    automatic: bool,
}

impl Paddle {
    pub fn new(side: Side, automatic: bool) -> Self {
        let half_width = WIDTH / 2.0;

        let pos = match side {
            Side::Left => Vec2::new(-1.0 + half_width + MARGIN, 0.0),
            Side::Right => Vec2::new(1.0 - half_width - MARGIN, 0.0),
        };

        Self {
            bounding_box: BoundingBox {
                pos,
                size: Vec2::new(WIDTH, HEIGHT),
            },
            side,
            direction: Direction::None,
            automatic,
        }
    }

    pub fn update(&mut self, delta_time: std::time::Duration, ball: Ball) {
        if self.automatic {
            let ball_y = ball.pos().y;
            let paddle_y = self.bounding_box.pos.y;

            log::debug!(
                "Automatic paddle: ball = {}, paddle = {}",
                ball_y,
                paddle_y
            );

            if ball_y > paddle_y {
                self.direction = Direction::Down;
            } else if ball_y < paddle_y {
                self.direction = Direction::Up;
            } else {
                self.direction = Direction::None;
            }

            log::debug!("New direction: {:?}", self.direction);
        }

        let change = match self.direction {
            Direction::Down => SPEED * delta_time.as_secs_f64(),
            Direction::Up => -SPEED * delta_time.as_secs_f64(),
            Direction::None => return,
        };

        let min_y = -1.0 + self.bounding_box.size.y / 2.0;
        let max_y = 1.0 - self.bounding_box.size.y / 2.0;

        self.bounding_box.pos.y += change;
        self.bounding_box.pos.y = self.bounding_box.pos.y.clamp(min_y, max_y);
    }

    pub fn draw(&self, screen: Screen, context: &CanvasRenderingContext2d) {
        let (x, y) = self.bounding_box.top_left().to_screen_coordinates(screen);
        let (width, height) = self.bounding_box.size.to_screen_size(screen);
        let side = format!("{:?}", self.side).to_ascii_lowercase();

        log::debug!(
            "Drawing {side:?} paddle to x{} y{}, {}x{}",
            x,
            y,
            width,
            height
        );

        context.set_fill_style_str("white");
        context
            .round_rect_with_f64(x, y, width, height, 5.0)
            .unwrap();
        context.fill();
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
