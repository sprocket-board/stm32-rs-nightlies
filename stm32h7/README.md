# stm32h7
This crate provides an autogenerated API for access to STM32H7 peripherals.
The API is generated using [svd2rust] with patched svd files containing
extensive type-safe support. For more information please see the [main repo].

Refer to the [documentation] for full details.

[svd2rust]: https://github.com/rust-embedded/svd2rust
[main repo]: https://github.com/stm32-rs/stm32-rs
[documentation]: https://docs.rs/stm32h7/latest/stm32h7/

## Usage
Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.stm32h7]
version = "0.15.1"
features = ["stm32h735"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

In your code:

```rust
use stm32h7::stm32h735;

let mut peripherals = stm32h735::Peripherals::take().unwrap();
let gpioa = &peripherals.GPIOA;
gpioa.odr.modify(|_, w| w.odr0().set_bit());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.25.0/svd2rust/#peripheral-api

## Supported Devices

| Module | Devices | Links |
|:------:|:-------:|:-----:|
| stm32h735 | STM32H723, STM32H725, STM32H730, STM32H733, STM32H735 | [RM0468](https://www.st.com/resource/en/reference_manual/dm00603761.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h730-value-line.html) |
| stm32h743 | STM32H743 | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h743v | STM32H743V | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h747cm4 | STM32H745 (CM4), STM32H747 (CM4), STM32H755 (CM4), STM32H757 (CM4) | [RM0399](https://www.st.com/resource/en/reference_manual/dm00176879.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h747-757.html) |
| stm32h747cm7 | STM32H745 (CM7), STM32H747 (CM7), STM32H755 (CM7), STM32H757 (CM7) | [RM0399](https://www.st.com/resource/en/reference_manual/dm00176879.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h747-757.html) |
| stm32h753 | STM32H753 | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h753v | STM32H753V | [RM0433](https://www.st.com/resource/en/reference_manual/dm00314099.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h743-753.html) |
| stm32h7b3 | STM32H7A3, STM32H7B3, STM32H7B0 | [RM0455](https://www.st.com/resource/en/reference_manual/dm00463927.pdf), [st.com](https://www.st.com/en/microcontrollers-microprocessors/stm32h7a3-7b3.html) |