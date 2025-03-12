#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::{self as _, Board};
use panic_halt as _;

#[entry]
fn entry() -> ! {
    main()
}

fn main() -> ! {
    let mut board = Board::take().unwrap();
    board.display_pins.col1.set_high().unwrap();
    board.display_pins.row1.set_low().unwrap();
    loop {}
}
