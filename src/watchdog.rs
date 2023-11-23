//! Watchdog peripheral

use crate::pac::WWDT;
use embedded_hal::watchdog::{Watchdog, WatchdogEnable};
use fugit::MillisDurationU32 as MilliSeconds;

const KEY_READ_WRITE: u32 = 0x55AA_6699;
const KEY_READ_ONLY: u32 = 0x55AA_6698;
const MAX_CLKDIV: u32 = 0b1111;
/// Maximum prescaler
const MAX_CDV: u16 = 0xFFFF;
/// Maximum count-down value
const ROSC_KHZ: u32 = 16_000;

pub struct WindowWatchdog {
    wwdt: WWDT,
}

impl WindowWatchdog {
    /// Creates a new `WindowWatchdog` without starting it. Call `start` to start the watchdog.
    /// See `WatchdogEnable` and `Watchdog` for more info.
    pub fn new(wwdt: WWDT) -> Self {
        WindowWatchdog { wwdt }
    }

    /// Sets the watchdog timer timeout period. Max: 32768 ms
    pub fn setup(&self, timeout_ms: MilliSeconds) {
        assert!(timeout_ms.ticks() < (1 << 15), "Watchdog timeout to high");

        self.write_unprotect();

        // select fixed 16MHz clock
        self.wwdt
            .ctl
            .write(|w| w.clksel().variant(crate::pac::wwdt::ctl::CLKSEL_A::Roscclk));
        self.wwdt.cdctl.write(|w| unsafe { w.window().bits(0) });
        self.wwdt.ctl.write(|w| w.rsten().set_bit());

        // select fixed 16MHz clock
        let mut clkdiv: u8 = 0b1111;
        for i in 0..14 {
            if timeout_ms.ticks() <= (MAX_CLKDIV + 1) * (1 << i) / ROSC_KHZ {
                clkdiv = i;
                break;
            }
        }

        let max_period = Self::timeout_period(clkdiv, MAX_CDV);
        let max_cdv = u32::from(MAX_CDV);
        let cdv = (timeout_ms.ticks() * max_cdv / max_period).min(max_cdv) as u16;

        self.wwdt
            .ctl
            .modify(|_, w| unsafe { w.clkdiv().bits(clkdiv) });
        self.wwdt.cdctl.modify(|_, w| unsafe { w.cdv().bits(cdv) });
    }

    /// Returns the interval in ms
    pub fn interval(&self) -> MilliSeconds {
        let clkdiv = self.wwdt.ctl.read().clkdiv().bits();
        let cdv = self.wwdt.cdctl.read().cdv().bits();
        let ms = Self::timeout_period(clkdiv, cdv);
        MilliSeconds::from_ticks(ms)
    }

    /// Returns timeout period in milliseconds
    ///
    /// * `clkdiv` - Clock divider value
    /// * `cdv` - Count down value
    fn timeout_period(clkdiv: u8, cdv: u16) -> u32 {
        let divider: u32 = 2_u32.pow(clkdiv.into());

        (u32::from(cdv) + 1) * divider / ROSC_KHZ
    }

    fn write_unprotect(&self) {
        self.wwdt
            .lock
            .write(|w| unsafe { w.value().bits(KEY_READ_WRITE) });
    }

    fn write_protect(&self) {
        self.wwdt
            .lock
            .write(|w| unsafe { w.value().bits(KEY_READ_ONLY) });
    }

    pub fn start(&mut self, period: MilliSeconds) {
        self.setup(period);

        self.wwdt.ctl.write(|w| w.en().set_bit());

        self.write_protect();
    }

    pub fn feed(&mut self) {
        // value doesn't matter
        self.wwdt.clear.write(|w| unsafe { w.value().bits(0b0101) });
    }
}

impl WatchdogEnable for WindowWatchdog {
    type Time = MilliSeconds;

    fn start<T: Into<Self::Time>>(&mut self, period: T) {
        self.start(period.into())
    }
}

impl Watchdog for WindowWatchdog {
    fn feed(&mut self) {
        self.feed()
    }
}
