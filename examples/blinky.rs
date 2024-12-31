//! Blinks an LED

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
    defmt::info!("Starting Blinky!");

    let dp = pac::Peripherals::take().unwrap();
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd4.into_push_pull_output();

    loop {
        defmt::info!("Blink!");

        if led.is_set_high() {
            led.set_low()
        } else {
            led.set_high()
        }

        for _ in 0..5_000 {
            continue;
        }
    }
}
