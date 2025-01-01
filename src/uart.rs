use crate::pac::usarta::lcr::{PSEL, SBS, WLS};
use crate::time::{Bps, U32Ext};

pub type Parity = PSEL;
pub type StopBits = SBS;
pub type WordLength = WLS;

pub struct Config {
    baudrate: Bps,
    parity: Option<Parity>,
    stop_bits: StopBits,
    word_length: WordLength,
}

impl Config {
    /// Set the baudrate to a specific value
    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }

    /// Set parity to none
    pub fn parity_none(mut self) -> Self {
        self.parity = None;
        self
    }

    /// Set parity to even
    pub fn parity_even(mut self) -> Self {
        self.parity = Some(Parity::Even);
        self
    }

    /// Set parity to odd
    pub fn parity_odd(mut self) -> Self {
        self.parity = Some(Parity::Odd);
        self
    }

    /// Set the number of stop bits
    pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    /// Set word length
    pub fn word_length(mut self, word_length: WordLength) -> Self {
        self.word_length = word_length;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        let baudrate = 115_200_u32.bps();
        Config {
            baudrate,
            parity: None,
            stop_bits: StopBits::Stop1,
            word_length: WordLength::Length8,
        }
    }
}

impl From<Bps> for Config {
    fn from(baudrate: Bps) -> Config {
        Config {
            baudrate,
            ..Default::default()
        }
    }
}
