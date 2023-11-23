//! Reads the state of an input pin

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use pac25_hal as hal;
use panic_probe as _;

use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    defmt::println!("Starting reading.");

    let dp = pac::Peripherals::take().unwrap();
    let gpiod = dp.GPIOD.split();
    let mut input = gpiod.pd4.into_pull_down_input();

    loop {
        defmt::println!("State: {=bool}", input.is_high());
        for _ in 0..5_000 {
            continue;
        }
    }
}
