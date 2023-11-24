#![no_std]

#[cfg(feature = "device-selected")]
pub use embedded_hal as hal;

#[cfg(feature = "pac25140")]
pub use pac25::pac25140 as pac;

#[cfg(feature = "device-selected")]
pub mod crc;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod uart;

#[cfg(feature = "device-selected")]
mod sealed {
    pub trait Sealed {}
}
#[cfg(feature = "device-selected")]
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
