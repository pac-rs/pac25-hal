#![no_std]

pub use embedded_hal as hal;

pub use pac25::pac25140 as pac;

pub mod gpio;
pub mod prelude;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
