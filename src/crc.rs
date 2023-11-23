//! CRC Calculation Unit

use crate::pac::crc::ctl::{DATAWIDTH_A, POLYSEL_A};
use crate::pac::CRC;

/// Polynomial setting
pub enum Polynomial {
    Crc16Ccitt = POLYSEL_A::Crc16Ccitt as isize,
    Crc16Ibm = POLYSEL_A::Crc16Ibm as isize,
    Crc8DallasMaxim = POLYSEL_A::Crc8DallasMaxim as isize,
}

/// Data width setting
#[derive(PartialEq)]
pub enum DataWidth {
    Width32Bits = DATAWIDTH_A::Width32Bits as isize,
    Width8Bits = DATAWIDTH_A::Width8Bits as isize,
}

pub struct Crc {
    crc: CRC,
}

impl Crc {
    /// Creates a new `Crc` peripheral instance.
    pub fn new(crc: CRC, poly: Polynomial, width: DataWidth) -> Self {
        // configure crc type
        crc.ctl.write(|w| unsafe {
            w.polysel()
                .bits(poly as u8)
                .datawidth()
                .bit(width == DataWidth::Width8Bits)
        });

        Crc { crc }
    }

    /// Set the CRC seed value.
    ///
    /// In any CRC-16 configuration, the seed will be sourced from bits 0:15.
    /// In the CRC-8 configuration, the seed will be sourced from bits 0:7.
    pub fn seed(&self, seed: u16) {
        self.crc.seed.write(|w| unsafe { w.value().bits(seed) })
    }

    /// Return ownership of the peripheral.
    ///
    /// This can be used in cases where the peripheral needs to be reconfigured
    /// at runtime.
    pub fn release(self) -> CRC {
        self.crc
    }
}
