use crate::{
    ball::Ball,
    keyboard::Keyboard,
    microbit_state::MicrobitState,
    paddle::{Direction, Paddle, Side},
    screen::Screen,
};

const LEFT_UP_KEYS: &[&'static str] = &["w", "k"];
const LEFT_DOWN_KEYS: &[&'static str] = &["s", "j"];
const RIGHT_UP_KEYS: &[&'static str] = &["arrowup"];
const RIGHT_DOWN_KEYS: &[&'static str] = &["arrowdown"];

pub struct GameState {
    last_tick: web_time::Instant,
    ball: Ball,
    screen: Screen,
    right_paddle: Paddle,
    left_paddle: Paddle,
    keyboard: Keyboard,
    microbit_state: MicrobitState,
}

impl GameState {
    pub fn new(screen: Screen, keyboard: Keyboard) -> Self {
        let ball = Ball::new(screen);

        Self {
            last_tick: web_time::Instant::now(),
            ball,
            screen,
            right_paddle: Paddle::new(Side::Right, true),
            left_paddle: Paddle::new(Side::Left, false),
            keyboard,
            microbit_state: MicrobitState::default(),
        }
    }

    pub fn update(&mut self, screen: Screen) {
        self.screen = screen;

        let now = web_time::Instant::now();
        let delta_time = now - self.last_tick;
        self.last_tick = now;

        let left_paddle_direction = match (
            self.microbit_state.left_pressed,
            self.microbit_state.right_pressed,
        ) {
            (true, false) => Direction::Up,
            (false, true) => Direction::Down,
            _ => Direction::None,
        };
        let right_paddle_direction =
            self.get_direction(RIGHT_UP_KEYS, RIGHT_DOWN_KEYS);

        self.left_paddle.change_direction(left_paddle_direction);
        self.right_paddle.change_direction(right_paddle_direction);

        self.ball.update(
            screen,
            delta_time,
            self.right_paddle,
            self.left_paddle,
        );

        self.left_paddle.update(delta_time, self.ball);
        self.right_paddle.update(delta_time, self.ball);
    }

    pub fn ball(&self) -> Ball {
        self.ball
    }

    pub fn left_paddle(&self) -> Paddle {
        self.left_paddle
    }

    pub fn right_paddle(&self) -> Paddle {
        self.right_paddle
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn get_direction(
        &self,
        up_keys: &[&'static str],
        down_keys: &[&'static str],
    ) -> Direction {
        let up = self.keyboard.is_any_pressed(up_keys);
        let down = self.keyboard.is_any_pressed(down_keys);

        return match (down, up) {
            (false, true) => Direction::Up,
            (true, false) => Direction::Down,
            _ => Direction::None,
        };
    }

    pub fn update_microbit_state(&mut self, state: MicrobitState) {
        self.microbit_state = state;
    }
}
