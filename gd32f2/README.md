# gd32f2
This crate provides an autogenerated API for access to GD32F2 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/japaric/svd2rust
[main repo]: https://github.com/gd32-rust/gd32-rs
[documentation]: https://docs.rs/gd32f2/latest/gd32f2/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.gd32f2]
version = "0.9.1"
features = ["gd32f205", "rt", "critical-section"]
```

The `rt` feature is optional and brings in support for `cortex-m-rt`.

In your code:

```rust
use gd32f2::gd32f205;

let mut peripherals = gd32f205::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.32.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| gd32f205 | GD32F205 | [GD32F205](https://www.gigadevice.com/datasheet/gd32f205xxxx-datasheet/), [gigadevice.com](https://www.gigadevice.com/products/microcontrollers/gd32/arm-cortex-m3/performance-line/gd32f205-series/) |
| gd32f207 | GD32F207 | [GD32F207](https://www.gigadevice.com/datasheet/gd32f207xxxx-datasheet/), [gigadevice.com](https://www.gigadevice.com/products/microcontrollers/gd32/arm-cortex-m3/performance-line/gd32f207-series/) |
