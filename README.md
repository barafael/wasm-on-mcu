# hello-nucleo-h7xx

Rust [`probe-run`] + [`defmt`] starter project for STMicroElectronics [STM32H7 Nucleo-144 boards](https://www.st.com/content/st_com/en/search.html#q=nucleo-h7-t=tools-page=1).

[`probe-run`]: https://crates.io/crates/probe-run
[`defmt`]: https://github.com/knurling-rs/defmt

Based on the lovely: [knurling-rs/app-template](https://github.com/knurling-rs/app-template)


## Dependencies

### Install ARM target

    rustup target add thumbv7em-none-eabihf

### Install probe-run

    cargo install probe-run --version "~0.2.0"

### Install latest firmware for onboard STLink V3

Download updater: [stsw-link007](https://www.st.com/content/st_com/en/products/development-tools/software-development-tools/stm32-software-development-tools/stm32-programmers/stsw-link007.html)


## Generate project

    cargo install cargo-generate

    cargo generate \
        --git https://github.com/antoinevg/hello-nucleo-h7xx \
        --name your-project-name


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
