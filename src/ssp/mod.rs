mod config;

pub use config::*;
use pac25::pac25140::SSPA;

use crate::scc::Scc;

pub trait SspExt {
    fn spi(self, scc: &mut Scc, config: Config) -> Spi;
}

impl SspExt for SSPA {
    /// Creates an SPI interface.
    fn spi(self, scc: &mut Scc, config: Config) -> Spi {
        // configure peripheral into ssp mode
        scc.reg.ccsctl().modify(|_, w| w.usamode().ssp());

        // configure clock
        self.clk()
            .modify(|_, w| unsafe { w.m().bits(config.m).n().bits(config.n) });

        // configure SPI
        self.con().modify(|_, w| unsafe {
            w.frf()
                .variant(config.frame_format)
                .ms()
                .variant(config.mode)
                .lsb_first()
                .bit(config.lsb_first)
                .lbm()
                .bit(config.loopback)
                .cph()
                .variant(config.clock_out_phase)
                .cpo()
                .variant(config.clock_polarity)
                .dss()
                .bits(config.data_size - 1)
                .sod()
                .variant(config.slave_output_drive)
        });

        // clear interrupts
        self.clr()
            .modify(|_, w| w.rtic().clear_bit().roic().clear_bit());

        // enable
        self.con().modify(|_, w| w.en().set_bit());

        Spi { reg: self }
    }
}

/// SPI interface.
pub struct Spi {
    reg: SSPA,
}

impl Spi {
    /// Transmit one or more bytes.
    pub fn transmit(&mut self, buf: &[u16]) {
        for byte in buf {
            while self.reg.stat().read().tnf().bit_is_clear() {}
            self.reg
                .dat()
                .modify(|_, w| unsafe { w.bits(*byte as u32) });
        }
    }

    /// Receive data.
    pub fn receive(&mut self, buf: &mut [u16]) {
        for byte in buf {
            while self.reg.stat().read().rne().bit_is_clear() {}
            *byte = self.reg.dat().read().bits() as u16;
        }
    }
}
