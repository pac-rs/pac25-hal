# pac25-hal

pac25-hal contains a multi device hardware abstraction on top of the peripheral access API for the Qorvo PAC25 series microcontrollers. The selection of the MCU is done by features gates, typically specified by board support crates. The only currently supported configuration is `pac25140`.

The idea behind this crate is to gloss over the slight differences in the various peripherals available on those MCUs so a HAL can be written for all chips in that same family without having to cut and pase dcrates for every single model.

# Other optional features

* `defmt` — implementation of `defmt::Format` for public enums and structures. See [defmt](https://crates.io/crates/defmt).

Collaboration on this crate is highly welcome as are pull requests!

This create is inspired by the great work the folks over at [stm32-rs](https://github.com/stm32-rs).
