#![allow(unsafe_code)]

use core::cell::RefCell;
use cortex_m::interrupt::{Mutex, free};
use microbit::hal::gpiote::Gpiote;
use microbit::pac::interrupt;
use microbit::{
    board::Buttons,
    pac::{self, GPIOTE},
};
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Default, Debug)]
pub struct ButtonState {
    left_pressed: bool,
    right_pressed: bool,
}

pub static GPIO: Mutex<RefCell<Option<Gpiote>>> =
    Mutex::new(RefCell::new(None));
pub static BUTTON_STATE: Mutex<RefCell<Option<ButtonState>>> =
    Mutex::new(RefCell::new(None));

pub(crate) fn init_buttons(board_gpiote: GPIOTE, board_buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    let a_button = board_buttons.button_a.degrade();
    let b_button = board_buttons.button_b.degrade();

    let channel0 = gpiote.channel0();
    channel0.input_pin(&a_button).hi_to_lo().enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1.input_pin(&a_button).lo_to_hi().enable_interrupt();
    channel1.reset_events();

    let channel2 = gpiote.channel2();
    channel2.input_pin(&b_button).hi_to_lo().enable_interrupt();
    channel2.reset_events();

    let channel3 = gpiote.channel3();
    channel3.input_pin(&b_button).lo_to_hi().enable_interrupt();
    channel3.reset_events();

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
            let a_released = gpiote.channel1().is_event_triggered();
            let b_pressed = gpiote.channel2().is_event_triggered();
            let b_released = gpiote.channel3().is_event_triggered();

            let mut state = BUTTON_STATE.borrow(cs).borrow_mut();
            let state: &mut ButtonState = state.get_or_insert_default();

            if a_pressed {
                state.left_pressed = true;
            }

            if a_released {
                state.left_pressed = false;
            }

            if b_pressed {
                state.right_pressed = true;
            }

            if b_released {
                state.right_pressed = false;
            }

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
            gpiote.channel2().reset_events();
            gpiote.channel3().reset_events();
        }
    });
}

pub fn get_button_state() -> Option<ButtonState> {
    free(|cs| *BUTTON_STATE.borrow(cs).borrow())
}
