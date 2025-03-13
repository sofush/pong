#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod buttons_setup;
mod serial_setup;

use buttons_setup::{ButtonState, init_buttons};
use cortex_m_rt::entry;
use embedded_hal_nb::serial::Write;
use microbit::hal::{
    Uarte,
    uarte::{Baudrate, Parity},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use serial_setup::UartePort;

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

    let mut latest_state = ButtonState::default();

    loop {
        let button_state = buttons_setup::get_button_state();

        let Some(button_state) = button_state else {
            continue;
        };

        if button_state == latest_state {
            continue;
        }

        latest_state = button_state;

        let Ok(msg) = serde_json_core::to_string::<_, 100>(&button_state)
        else {
            rprintln!("Error");
            write!(serial, "Error").unwrap();
            nb::block!(serial.flush()).unwrap();
            continue;
        };

        use core::fmt::Write;
        write!(serial, "{msg}\r\n").unwrap();
        nb::block!(serial.flush()).unwrap();
    }
}
