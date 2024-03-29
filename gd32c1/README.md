# gd32c1
This crate provides an autogenerated API for access to GD32C1 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/japaric/svd2rust
[main repo]: https://github.com/gd32-rust/gd32-rs
[documentation]: https://docs.rs/gd32c1/latest/gd32c1/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.gd32c1]
version = "0.9.1"
features = ["gd32c103", "rt", "critical-section"]
```

The `rt` feature is optional and brings in support for `cortex-m-rt`.

In your code:

```rust
use gd32c1::gd32c103;

let mut peripherals = gd32c103::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| gd32c103 | GD32C103 | [GD32C103](https://gd32mcu.com/download/down/document_id/269/path_type/1), [gigadevice.com](https://www.gigadevice.com/products/microcontrollers/gd32/arm-cortex-m4/value-line/gd32c103-series/) |
| gd32c113 | GD32C113 | [GD32C103](https://gd32mcu.com/download/down/document_id/382/path_type/1), [gigadevice.com](https://www.gigadevice.com/products/microcontrollers/gd32/arm-cortex-m4/value-line/gd32c113-series/) |
