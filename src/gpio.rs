use crate::hal::digital::v2::PinState;
use core::fmt;
use core::marker::PhantomData;
mod convert;

pub use Input as DefaultMode;

/// A filler pin type
#[derive(Debug, Default)]
pub struct NoPin<Otype = PushPull>(PhantomData<Otype>);
impl<Otype> NoPin<Otype> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The Parts to split the GPIO peripheral into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

/// Id, port and mode for any pin
pub trait PinExt {
    /// Current pin mode
    type Mode;
    /// Pin number
    fn pin_id(&self) -> u8;
    /// Port number starting from 0
    fn port_id(&self) -> u8;
}

/// Some alternate mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Alternate<const A: u8, Otype = PushPull>(PhantomData<Otype>);

/// Input mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Input;

/// Pull setting for an input
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pull {
    /// Floating
    None,
    /// Pulled up
    Up,
    /// Pulled down
    Down,
}

/// Open drain input or output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OpenDrain;

/// Output mode (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Output<MODE = PushPull> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PushPull;

/// JTAG/SWD mode (type state)
pub type Debugger = Alternate<0b011, PushPull>;

pub(crate) mod marker {
    /// Marker trait that show if `ExtiPin` can be implemented
    pub trait Interruptible {}
    /// Marker trait for readable pin modes
    pub trait Readable {}
    /// Marker trait for slew rate configurable pin modes
    pub trait OutputSpeed {}
    /// Marker trait for active pin modes
    pub trait Active {}
    /// Marker trait for all pin modes except alternate
    pub trait NotAlt {}
    /// Marker trait for pins with alternate function `A` mapping
    pub trait IntoAf<const A: u8> {}
}

impl marker::Readable for Input {}
impl marker::Readable for Output<OpenDrain> {}
impl marker::Active for Input {}

/// GPIO Pin drive strength
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DriveStrength {
    _6mA = 0b000,
    _8mA = 0b001,
    _11mA = 0b010,
    _14mA = 0b011,
    _17mA = 0b100,
    _20mA = 0b101,
    _22mA = 0b110,
    _25mA = 0b111,
}

macro_rules! af {
    ($($i:literal: $AFi:ident),+) => {
        $(
            #[doc = concat!("Alternate function ", $i, " (type state)" )]
            pub type $AFi<Otype = PushPull> = Alternate<$i, Otype>;
        )+
    };
}

af!(
    0: AF0,
    1: AF1,
    2: AF2,
    3: AF3,
    4: AF4,
    5: AF5,
    6: AF6,
    7: AF7
);

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `7`.
pub struct Pin<const P: char, const N: u8, MODE = DefaultMode> {
    _mode: PhantomData<MODE>,
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<const P: char, const N: u8, MODE> fmt::Debug for Pin<P, N, MODE> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "P{}{}<{}>",
            P,
            N,
            crate::stripped_type_name::<MODE>()
        ))
    }
}

#[cfg(feature = "defmt")]
impl<const P: char, const N: u8, MODE> defmt::Format for Pin<P, N, MODE> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P{}{}<{}>", P, N, crate::stripped_type_name::<MODE>());
    }
}

impl<const P: char, const N: u8, MODE> PinExt for Pin<P, N, MODE> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

pub trait PinPull: Sized {
    /// Set the internal pull-up and pull-down resistor
    fn set_internal_resistor(&mut self, resistor: Pull);

    #[inline(always)]
    fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }
}

impl<const P: char, const N: u8, MODE> PinPull for Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    #[inline(always)]
    fn set_internal_resistor(&mut self, resistor: Pull) {
        self.set_internal_resistor(resistor)
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Active,
{
    /// Set the internal pull-up and pull-down resistor
    pub fn set_internal_resistor(&mut self, resistor: Pull) {
        let offset = N;

        // for safety reasons we need to ensure the pull-up and pull-down are
        // not activated at the same time.

        unsafe {
            (*Gpio::<P>::scc_puen_ptr()).modify(|r, w| w.bits(r.bits() & !(1 << offset)));
            (*Gpio::<P>::scc_pden_ptr()).modify(|r, w| w.bits(r.bits() & !(1 << offset)));

            match resistor {
                Pull::Up => {
                    (*Gpio::<P>::scc_puen_ptr()).modify(|r, w| w.bits(r.bits() | (1 << offset)))
                }
                Pull::Down => {
                    (*Gpio::<P>::scc_pden_ptr()).modify(|r, w| w.bits(r.bits() | (1 << offset)))
                }
                Pull::None => {},
            }
        }
    }

    /// Set the internal pull-up and pull-down resistor
    pub fn internal_resistor(mut self, resistor: Pull) -> Self {
        self.set_internal_resistor(resistor);
        self
    }

    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Up)
        } else {
            self.internal_resistor(Pull::None)
        }
    }

    /// Enables / disables the internal pull down
    pub fn internal_pull_down(self, on: bool) -> Self {
        if on {
            self.internal_resistor(Pull::Down)
        } else {
            self.internal_resistor(Pull::None)
        }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).doset.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Gpio::<P>::ptr()).doclear.write(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).out.read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Gpio::<P>::ptr()).in_.read().bits() & (1 << N) == 0 }
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, Output<MODE>> {
    /// Drives the pin high
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    /// Drives the pin low
    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    /// Is the pin in drive high or low mode?
    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    /// Drives the pin high or low depending on the provided value
    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    /// Is the pin in drive high mode?
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    /// Is the pin in drive low mode?
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }

    /// Toggle pin output
    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

pub trait ReadPin {
    #[inline(always)]
    fn is_high(&self) -> bool {
        !self.is_low()
    }
    fn is_low(&self) -> bool;
}

impl<const P: char, const N: u8, MODE> ReadPin for Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    #[inline(always)]
    fn is_low(&self) -> bool {
        self.is_low()
    }
}

impl<const P: char, const N: u8, MODE> Pin<P, N, MODE>
where
    MODE: marker::Readable,
{
    /// Is the input pin high?
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    /// Is the input pin low?
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, [$($A:literal),*] $(, $MODE:ty)?),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::pac::$GPIOX;

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi $(<$MODE>)?,
                )+
            }

            impl super::GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            /*
            #[doc="Common type for "]
            #[doc=stringify!($GPIOX)]
            #[doc=" related pins"]
            pub type $PXn<MODE> = super::PartiallyErasedPin<$port_id, MODE>;
             */

            $(
                #[doc=stringify!($PXi)]
                #[doc=" pin"]
                pub type $PXi<MODE = super::DefaultMode> = super::Pin<$port_id, $i, MODE>;

                $(
                    impl<MODE> super::marker::IntoAf<$A> for $PXi<MODE> { }
                )*
            )+

        }

        pub use $gpiox::{ $($PXi,)+ };
    }
}
use gpio;

mod pac25;
pub use pac25::*;

struct Gpio<const P: char>;
impl<const P: char> Gpio<P> {
    const fn ptr() -> *const crate::pac::gpioa::RegisterBlock {
        match P {
            #[cfg(feature = "gpioa")]
            'A' => crate::pac::GPIOA::ptr(),
            #[cfg(feature = "gpiob")]
            'B' => crate::pac::GPIOB::ptr() as _,
            #[cfg(feature = "gpioc")]
            'C' => crate::pac::GPIOC::ptr() as _,
            #[cfg(feature = "gpiod")]
            'D' => crate::pac::GPIOD::ptr() as _,
            #[cfg(feature = "gpioe")]
            'E' => crate::pac::GPIOE::ptr() as _,
            #[cfg(feature = "gpiof")]
            'F' => crate::pac::GPIOF::ptr() as _,
            #[cfg(feature = "gpiog")]
            'G' => crate::pac::GPIOG::ptr() as _,
            _ => panic!("Unknown GPIO port"),
        }
    }

    fn scc_muxsel_ptr() -> *const crate::pac::scc::PAMUXSEL {
        unsafe {
            match P {
                #[cfg(feature = "gpioa")]
                'A' => (*crate::pac::SCC::ptr()).pamuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpiob")]
                'B' => (*crate::pac::SCC::ptr()).pbmuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpioc")]
                'C' => (*crate::pac::SCC::ptr()).pcmuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpiod")]
                'D' => (*crate::pac::SCC::ptr()).pdmuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpioe")]
                'E' => (*crate::pac::SCC::ptr()).pemuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpiof")]
                'F' => (*crate::pac::SCC::ptr()).pfmuxsel.as_ptr() as *const _,
                #[cfg(feature = "gpiog")]
                'G' => (*crate::pac::SCC::ptr()).pgmuxsel.as_ptr() as *const _,
                _ => panic!("Unknown GPIO port"),
            }
        }
    }

    fn scc_puen_ptr() -> *const crate::pac::scc::PAPUEN {
        unsafe {
            match P {
                #[cfg(feature = "gpioa")]
                'A' => (*crate::pac::SCC::ptr()).papuen.as_ptr() as *const _,
                #[cfg(feature = "gpiob")]
                'B' => (*crate::pac::SCC::ptr()).pbpuen.as_ptr() as *const _,
                #[cfg(feature = "gpioc")]
                'C' => (*crate::pac::SCC::ptr()).pcpuen.as_ptr() as *const _,
                #[cfg(feature = "gpiod")]
                'D' => (*crate::pac::SCC::ptr()).pdpuen.as_ptr() as *const _,
                #[cfg(feature = "gpioe")]
                'E' => (*crate::pac::SCC::ptr()).pepuen.as_ptr() as *const _,
                #[cfg(feature = "gpiof")]
                'F' => (*crate::pac::SCC::ptr()).pfpuen.as_ptr() as *const _,
                #[cfg(feature = "gpiog")]
                'G' => (*crate::pac::SCC::ptr()).pgpuen.as_ptr() as *const _,
                _ => panic!("Unknown GPIO port"),
            }
        }
    }

    fn scc_pden_ptr() -> *const crate::pac::scc::PAPDEN {
        unsafe {
            match P {
                #[cfg(feature = "gpioa")]
                'A' => (*crate::pac::SCC::ptr()).papden.as_ptr() as *const _,
                #[cfg(feature = "gpiob")]
                'B' => (*crate::pac::SCC::ptr()).pbpden.as_ptr() as *const _,
                #[cfg(feature = "gpioc")]
                'C' => (*crate::pac::SCC::ptr()).pcpden.as_ptr() as *const _,
                #[cfg(feature = "gpiod")]
                'D' => (*crate::pac::SCC::ptr()).pdpden.as_ptr() as *const _,
                #[cfg(feature = "gpioe")]
                'E' => (*crate::pac::SCC::ptr()).pepden.as_ptr() as *const _,
                #[cfg(feature = "gpiof")]
                'F' => (*crate::pac::SCC::ptr()).pfpden.as_ptr() as *const _,
                #[cfg(feature = "gpiog")]
                'G' => (*crate::pac::SCC::ptr()).pgpden.as_ptr() as *const _,
                _ => panic!("Unknown GPIO port"),
            }
        }
    }
}
