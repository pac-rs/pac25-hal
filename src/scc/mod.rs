pub mod config;

pub use config::*;

use crate::time::Hertz;
use fugit::RateExtU32;
use pac25::pac25140::SCC;

/// Clock frequencies.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    /// System clock frequency.
    pub sclk: Hertz,
    /// AHB clock frequency.
    pub hclk: Hertz,
    /// Auxiliary clock frequency.
    pub aclk: Hertz,
    /// Peripheral clock frequency.
    pub pclk: Hertz,
}

impl Default for Clocks {
    fn default() -> Self {
        Self {
            sclk: Hertz::Hz(0),
            hclk: Hertz::Hz(0),
            aclk: Hertz::Hz(0),
            pclk: Hertz::Hz(0),
        }
    }
}

/// System and clock control.
pub struct Scc {
    pub clocks: Clocks,
    pub(crate) reg: SCC,
}

impl Scc {
    /// Apply clock configuration.
    pub fn freeze(mut self, config: Config) -> Self {
        // Configure FRCLK
        let frclk = match config.frclk_mux {
            ClockSource::RingOsc => {
                self.reg.ccsctl().modify(|_, w| w.roscen().set_bit());
                self.reg.ccsctl().modify(|_, w| w.frclkmuxsel().rosc());
                16_u32.MHz()
            }
            ClockSource::RefClock => {
                self.reg.ccsctl().modify(|_, w| w.frclkmuxsel().clkref());
                4_u32.MHz()
            }
            ClockSource::External(freq) => {
                self.reg.ccsctl().modify(|_, w| w.frclkmuxsel().extclk());
                freq
            }
        };

        // Configure SCLK
        let sclk = match config.sclk_mux {
            SystemClock::FRCLK => {
                self.reg.ccsctl().modify(|_, w| w.sclkmuxsel().frclk());
                frclk
            }
            SystemClock::PLLCLK => {
                self.reg.ccspllctl().modify(|_, w| unsafe {
                    w.plloutdiv()
                        .variant(config.pll.outdiv)
                        .pllindiv()
                        .bits(config.pll.indiv)
                        .pllfbdiv()
                        .bits(config.pll.fbdiv)
                        .pllbp()
                        .bit(config.pll.bypass)
                        .pllen()
                        .bit(config.pll.enabled)
                });

                while self.reg.ccspllctl().read().plllock().bit_is_clear() {}

                self.reg.ccsctl().modify(|_, w| w.sclkmuxsel().pllclk());

                config.pll.out_freq(frclk)
            }
        };

        // Configure HCLK
        let hdiv = config.hclk as u32 + 1;
        let hclk = Hertz::Hz(sclk.to_Hz() / hdiv);
        self.reg
            .ccsctl()
            .modify(|_, w| w.hclkdiv().variant(config.hclk));

        // Configure ACLK
        let adiv = config.aclk as u32 + 1;
        let aclk = Hertz::Hz(sclk.to_Hz() / adiv);
        self.reg
            .ccsctl()
            .modify(|_, w| w.aclkdiv().variant(config.aclk));

        // Configure PCLK
        let pdiv = config.pclk as u32 + 1;
        let pclk = Hertz::Hz(hclk.to_Hz() / pdiv);
        self.reg
            .ccsctl()
            .modify(|_, w| w.pclkdiv().variant(config.pclk));

        self.clocks = Clocks {
            sclk,
            hclk,
            aclk,
            pclk,
        };

        self
    }
}

pub trait SccExt {
    /// Constains the `SCC` peripheral.
    fn constain(self) -> Scc;

    /// Constains the `SCC` peripheral and applies a clock configuration.
    fn freeze(self, config: Config) -> Scc;
}

impl SccExt for SCC {
    fn constain(self) -> Scc {
        Scc {
            clocks: Clocks::default(),
            reg: self,
        }
    }

    fn freeze(self, config: Config) -> Scc {
        self.constain().freeze(config)
    }
}
