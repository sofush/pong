#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal_nb::serial::Write;
use microbit::hal::{
    Uarte,
    uarte::{Baudrate, Parity},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod serial_setup;
use serial_setup::UartePort;

mod buttons_setup;
use buttons_setup::init_buttons;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = microbit::Board::take().unwrap();

    init_buttons(board.GPIOTE, board.buttons);

    let mut serial = {
        let uarte = Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(uarte)
    };

    loop {
        let event = buttons_setup::get_button_event(true);

        let Some(event) = event else {
            continue;
        };

        let msg = match event {
            buttons_setup::Button::Left => "Left",
            buttons_setup::Button::Right => "Right",
            buttons_setup::Button::Both => "Both",
        };

        use core::fmt::Write;
        write!(serial, "{msg}\r\n").unwrap();
        nb::block!(serial.flush()).unwrap();
    }
}
