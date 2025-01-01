//! Clock Control System

use crate::pac::scc::ccsctl::{ACLKDIV, HCLKDIV, PCLKDIV};
use crate::pac::scc::ccspllctl::PLLOUTDIV;
use crate::time::Hertz;

/// Clock source.
#[derive(Debug, Clone, Copy)]
pub enum ClockSource {
    /// 16MHz ring oscillator.
    RingOsc,
    /// 4MHz always running reference clock.
    RefClock,
    /// External clock input.
    External(Hertz),
}

/// System clock selection.
#[derive(Debug, Clone, Copy)]
pub enum SystemClock {
    /// Free-running clock.
    FRCLK = 0b0,
    /// PLL clock.
    PLLCLK = 0b1,
}

/// HCLK prescaler.
pub type HClockPrescaler = HCLKDIV;
/// ACLK prescaler.
pub type AClockPrescaler = ACLKDIV;
/// PCLK prescaler.
pub type PClockPrescaler = PCLKDIV;

/// Clock tree configuration.
#[derive(Debug)]
pub struct Config {
    pub(crate) hclk: HClockPrescaler,
    pub(crate) aclk_enable: bool,
    pub(crate) aclk: AClockPrescaler,
    pub(crate) pclk_enbable: bool,
    pub(crate) pclk: PClockPrescaler,
    pub(crate) sclk_mux: SystemClock,
    pub(crate) rosc_enable: bool,
    pub(crate) frclk_mux: ClockSource,
    pub pll: PllConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hclk: HClockPrescaler::SclkDiv1,
            aclk_enable: true,
            aclk: AClockPrescaler::SclkDiv1,
            pclk_enbable: true,
            pclk: PClockPrescaler::HclkDiv1,
            sclk_mux: SystemClock::FRCLK,
            rosc_enable: true,
            frclk_mux: ClockSource::RingOsc,
            pll: PllConfig::default(),
        }
    }
}

impl Config {
    /// Set the HCLK divider.
    pub fn hclk(mut self, value: HClockPrescaler) -> Self {
        self.hclk = value;
        self
    }

    /// Enable or disable the ACLK domain.
    pub fn aclk_enable(mut self, enabled: bool) -> Self {
        self.aclk_enable = enabled;
        self
    }

    /// Set the ACLK divider.
    pub fn aclk(mut self, value: AClockPrescaler) -> Self {
        self.aclk = value;
        self
    }

    /// Enable or disable the PCLK domain.
    pub fn pclk_enable(mut self, enabled: bool) -> Self {
        self.pclk_enbable = enabled;
        self
    }

    /// Set the PCLK divider.
    pub fn pclk(mut self, value: PClockPrescaler) -> Self {
        self.pclk = value;
        self
    }

    /// Set the SCLK input.
    pub fn sclk(mut self, mux: SystemClock) -> Self {
        self.sclk_mux = mux;
        self
    }

    /// Enable or disable the ROSC.
    pub fn rosc_enable(mut self, enabled: bool) -> Self {
        self.rosc_enable = enabled;
        self
    }

    /// Set the system clock source.
    pub fn frclk(mut self, mux: ClockSource) -> Self {
        self.frclk_mux = mux;
        self
    }

    /// Set the PLL configuration.
    pub fn pll(mut self, config: PllConfig) -> Self {
        self.pll = config;
        self
    }
}

/// PLL output divider.
pub type PllOutPrescaler = PLLOUTDIV;

/// PLL configuration.
#[derive(Debug)]
pub struct PllConfig {
    pub(crate) fbdiv: u16,
    pub(crate) indiv: u8,
    pub(crate) outdiv: PllOutPrescaler,
    pub(crate) bypass: bool,
    pub(crate) enabled: bool,
}

impl Default for PllConfig {
    fn default() -> Self {
        Self {
            fbdiv: 0,
            indiv: 0,
            outdiv: PllOutPrescaler::Div1,
            bypass: false,
            enabled: false,
        }
    }
}

impl PllConfig {
    /// Set the feedback divider.
    ///
    /// Must be between 4 and 16383.
    pub fn fbdiv(mut self, divider: u16) -> Self {
        assert!(divider >= 4);
        assert!(divider <= 16383);
        self.fbdiv = divider;
        self
    }

    /// Set the input divider.
    ///
    /// Must be between 1 and 15.
    pub fn indiv(mut self, divider: u8) -> Self {
        assert!(divider >= 1);
        assert!(divider <= 15);
        self.indiv = divider;
        self
    }

    /// Set the output divider.
    pub fn outdiv(mut self, divider: PllOutPrescaler) -> Self {
        self.outdiv = divider;
        self
    }

    /// Enable or disable bypassing the PLL.
    pub fn bypass(mut self, enabled: bool) -> Self {
        self.bypass = enabled;
        self
    }

    /// Enable or disable the PLL
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Calculator the voltage controlled oscillator output frequency.
    pub fn vco_freq(&self, input: Hertz) -> Hertz {
        let fbdiv = self.fbdiv as f32;
        let indiv = self.indiv as f32;
        let vco = input.to_Hz() as f32 * (fbdiv / indiv);
        Hertz::Hz(vco as u32)
    }

    /// Calculate the PLL output frequency.
    pub fn out_freq(&self, input: Hertz) -> Hertz {
        let outdiv = self.outdiv as u32 + 1;
        let vco = self.vco_freq(input).to_Hz() as f32;
        let out = vco / outdiv as f32;
        Hertz::Hz(out as u32)
    }
}
