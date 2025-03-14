use crate::{
    ball::Ball,
    paddle::{Paddle, Side},
    screen::Screen,
};

pub struct GameState {
    last_tick: web_time::Instant,
    ball: Ball,
    screen: Screen,
    right_paddle: Paddle,
    left_paddle: Paddle,
}

impl GameState {
    pub fn new(screen: Screen) -> Self {
        let ball = Ball::new(screen);

        Self {
            last_tick: web_time::Instant::now(),
            ball,
            screen,
            right_paddle: Paddle::new(Side::Right),
            left_paddle: Paddle::new(Side::Left),
        }
    }

    pub fn update(&mut self, screen: Screen) {
        self.screen = screen;

        let now = web_time::Instant::now();
        let delta_time = now - self.last_tick;
        self.last_tick = now;

        self.left_paddle.update(delta_time);
        self.right_paddle.update(delta_time);
        self.ball.update(
            screen,
            delta_time,
            self.right_paddle,
            self.left_paddle,
        );
    }

    pub fn ball(&self) -> Ball {
        self.ball
    }

    pub fn left_paddle(&self) -> Paddle {
        self.left_paddle
    }

    pub fn left_paddle_mut(&mut self) -> &mut Paddle {
        &mut self.left_paddle
    }

    pub fn right_paddle(&self) -> Paddle {
        self.right_paddle
    }

    pub fn right_paddle_mut(&mut self) -> &mut Paddle {
        &mut self.right_paddle
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }
}
