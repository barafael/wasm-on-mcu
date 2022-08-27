# Run WASM on MCU

For defmt etc:

Based on the lovely: [knurling-rs/app-template](https://github.com/knurling-rs/app-template)

## Dependencies

### Install ARM target

    rustup target add thumbv7em-none-eabihf

### Install probe-run

    cargo install probe-run --version "~0.2.0"

### Install latest firmware for onboard STLink V3

Download updater: [stsw-link007](https://www.st.com/content/st_com/en/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-programmers/stsw-link007.html)

### Run on STM32F411CEUx

    cargo run -b binary

## License

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
