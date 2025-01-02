use pac25::pac25140::sspa::con::{CPH, CPO, FRF, MS, SOD};

/// SPI configuration.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub(crate) m: u8,
    pub(crate) n: u8,
    pub(crate) frame_format: FRF,
    pub(crate) mode: MS,
    pub(crate) lsb_first: bool,
    pub(crate) loopback: bool,
    pub(crate) clock_out_phase: CPH,
    pub(crate) clock_polarity: CPO,
    pub(crate) data_size: u8,
    pub(crate) slave_output_drive: SOD,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            m: 2,
            n: 254,
            frame_format: FRF::Spi,
            mode: MS::Master,
            lsb_first: false,
            loopback: false,
            clock_out_phase: CPH::FirstEdge,
            clock_polarity: CPO::ActiveHigh,
            data_size: 8,
            slave_output_drive: SOD::Disabled,
        }
    }
}

impl Config {
    /// Set clock M value.
    pub fn m(mut self, value: u8) -> Self {
        self.m = value;
        self
    }

    /// Set clock N value.
    pub fn n(mut self, value: u8) -> Self {
        self.n = value;
        self
    }

    /// Set frame fromat.
    pub fn frame_format(mut self, format: FRF) -> Self {
        self.frame_format = format;
        self
    }

    /// Set master/slave mode.
    pub fn mode(mut self, mode: MS) -> Self {
        self.mode = mode;
        self
    }

    /// Set endian order.
    pub fn lsb_first(mut self, first: bool) -> Self {
        self.lsb_first = first;
        self
    }

    /// Set loopback mode.
    pub fn loopback(mut self, enabled: bool) -> Self {
        self.loopback = enabled;
        self
    }

    /// Set clock output phase.
    pub fn clock_out_phase(mut self, phase: CPH) -> Self {
        self.clock_out_phase = phase;
        self
    }

    /// Set clock polarity.
    pub fn clock_polarity(mut self, polarity: CPO) -> Self {
        self.clock_polarity = polarity;
        self
    }

    /// Set data size in bits.
    ///
    /// Must be between 4 and 32 bits.
    pub fn data_size(mut self, size: u8) -> Self {
        assert!(size >= 4);
        assert!(size <= 32);

        self.data_size = size;
        self
    }
}
