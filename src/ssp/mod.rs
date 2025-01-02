mod config;

pub use config::*;
use embedded_hal::spi::{ErrorKind, ErrorType, SpiBus};
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

impl ErrorType for Spi {
    type Error = ErrorKind;
}

impl SpiBus<u16> for Spi {
    fn read(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        for byte in words {
            // wait until RX FIFO has data
            while self.reg.stat().read().rne().bit_is_clear() {}
            // read word
            *byte = self.reg.dat().read().bits() as u16;
        }

        Ok(())
    }

    fn write(&mut self, words: &[u16]) -> Result<(), Self::Error> {
        for byte in words {
            // wait until TX FIFO is not full
            while self.reg.stat().read().tnf().bit_is_clear() {}
            // write word
            self.reg
                .dat()
                .modify(|_, w| unsafe { w.bits(*byte as u32) });
        }

        Ok(())
    }

    fn transfer(&mut self, read: &mut [u16], write: &[u16]) -> Result<(), Self::Error> {
        self.write(write)?;
        self.read(read)?;
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u16]) -> Result<(), Self::Error> {
        self.write(words)?;
        self.read(words)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // wait until TX FIFO is empty
        while self.reg.stat().read().tfe().bit_is_clear() {}
        Ok(())
    }
}
