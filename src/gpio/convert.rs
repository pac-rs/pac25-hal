use super::*;

impl<const P: char, const N: u8, MODE: PinMode> Pin<P, N, MODE> {
    /// Configures the pin to operate as a input pin
    pub fn into_input(self) -> Pin<P, N, Input> {
        self.into_mode()
    }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::None)
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::Down)
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(self) -> Pin<P, N, Input> {
        self.into_mode().internal_resistor(Pull::Up)
    }

    /// Configures the pin to operate as an open drain output pin
    /// Initial state will be low.
    pub fn into_open_drain_output(self) -> Pin<P, N, Output<OpenDrain>> {
        self.into_mode()
    }

    /// Configures the pin to operate as an open-drain output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_open_drain_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<OpenDrain>> {
        self._set_state(initial_state);
        self.into_mode()
    }

    /// Configures the pin to operate as an push pull output pin
    /// Initial state will be low.
    pub fn into_push_pull_output(mut self) -> Pin<P, N, Output<PushPull>> {
        self._set_low();
        self.into_mode()
    }

    /// Configures the pin to operate as an push-pull output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_push_pull_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<P, N, Output<PushPull>> {
        self._set_state(initial_state);
        self.into_mode()
    }

    /// Puts `self` into mode `M`.
    ///
    /// This violates the type state constraints from `MODE`, so callers must
    /// ensure they use this properly.
    #[inline(always)]
    pub(super) fn mode<M: PinMode>(&mut self) {
        change_mode!((*Gpio::<P>::ptr()), N);
    }

    #[inline(always)]
    /// Converts pin into specified mode
    pub fn into_mode<M: PinMode>(mut self) -> Pin<P, N, M> {
        self.mode::<M>();
        Pin::new()
    }
}

macro_rules! change_mode {
    ($block:expr, $N:ident) => {
        let offset = 2 * $N;
        unsafe {
            if MODE::MODE != M::MODE {
                $block
                    .mode()
                    .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (M::MODE << offset)));
            }
        }
    };
}
use change_mode;

/// Marker trait for valid pin modes (type state).
///
/// It can not be implemented by outside types.
pub trait PinMode: crate::Sealed {
    // These constants are used to implement the pin configuration code.
    // They are not part of public API.

    #[doc(hidden)]
    const MODE: u32 = u32::MAX;
}

impl crate::Sealed for Input {}
impl PinMode for Input {
    const MODE: u32 = 0b11;
}

impl<Otype> crate::Sealed for Output<Otype> {}
impl PinMode for Output<OpenDrain> {
    const MODE: u32 = 0b10;
}

impl PinMode for Output<PushPull> {
    const MODE: u32 = 0b01;
}
