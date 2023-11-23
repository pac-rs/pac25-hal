//! Example watchdog use

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use defmt_rtt as _;
use pac25_hal as hal;
use panic_halt as _;

use hal::pac;
use hal::watchdog::WindowWatchdog;
use fugit::MillisDurationU32 as MilliSeconds;

#[entry]
fn main() -> ! {
    defmt::println!("Reset!");

    let dp = pac::Peripherals::take().unwrap();
    let mut wd = WindowWatchdog::new(dp.WWDT);

    wd.start(MilliSeconds::millis(1000));

    loop {
        wd.feed();
        for _ in 0..5_000 {
            continue;
        }
    }
}
