#![allow(unsafe_code)]

use core::cell::RefCell;
use cortex_m::interrupt::{Mutex, free};
use microbit::hal::gpiote::Gpiote;
use microbit::pac::interrupt;
use microbit::{
    board::Buttons,
    pac::{self, GPIOTE},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
    Both,
}

pub static GPIO: Mutex<RefCell<Option<Gpiote>>> =
    Mutex::new(RefCell::new(None));
pub static BUTTON_STATE: Mutex<RefCell<Option<Button>>> =
    Mutex::new(RefCell::new(None));

pub(crate) fn init_buttons(board_gpiote: GPIOTE, board_buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board_buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board_buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    free(move |cs| {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }

        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let new_state = match (a_pressed, b_pressed) {
                (true, false) => Some(Button::Left),
                (false, true) => Some(Button::Right),
                (true, true) => Some(Button::Both),
                _ => None,
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            *BUTTON_STATE.borrow(cs).borrow_mut() = new_state;
        }
    });
}

pub fn get_button_event(reset: bool) -> Option<Button> {
    free(|cs| {
        let button = *BUTTON_STATE.borrow(cs).borrow();

        if reset {
            *BUTTON_STATE.borrow(cs).borrow_mut() = None
        }

        button
    })
}
