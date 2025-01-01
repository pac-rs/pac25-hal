//! Blinks an LED

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use pac25_hal as hal;
use pac25_hal::scc::{ClockSource, Config, PllConfig, PllOutPrescaler, SystemClock};
use panic_probe as _;

use hal::pac;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    defmt::info!("Starting Blinky!");

    let dp = pac::Peripherals::take().unwrap();
    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd4.into_push_pull_output();

    // Set the system clock to 150MHz
    let _scc = dp.SCC.freeze(
        Config::default()
            .frclk(ClockSource::RefClock)
            .sclk(SystemClock::PLLCLK)
            .pll(
                PllConfig::default()
                    .enabled(true)
                    .indiv(1)
                    .fbdiv(75)
                    .outdiv(PllOutPrescaler::Div2),
            ),
    );

    loop {
        led.toggle();

        for _ in 0..100_000 {
            continue;
        }
    }
}
