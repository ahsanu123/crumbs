# agnostic_mmc

The default library is `no_std` and MCU-independent. SD-card access uses
`embedded-hal` SPI and delay traits; the application provides the HAL, executor,
and Embassy time driver for its board.

```sh
cargo check --no-default-features
```

## Optional ESP backend

No chip, target, or toolchain is selected by default. The `esp` feature enables
the ESP Wi-Fi backend; select exactly one chip feature to forward the chip
selection to both `esp-hal` and `esp-radio`:

`esp32`, `esp32c2`, `esp32c3`, `esp32c5`, `esp32c6`, `esp32c61`,
`esp32s2`, or `esp32s3`.

Only Wi-Fi-capable chips supported by these dependency versions are listed.
ESP32-H2 has no Wi-Fi and is not supported by this backend. Chip features are
mutually exclusive: do not use `--all-features` for an embedded build.

ESP dependencies and source are gated to bare-metal Xtensa and RISC-V targets.
Host builds omit the backend even when an ESP feature is enabled. Other MCUs
can use the portable SD-card library without any ESP feature.

For ESP32-S3, use the Espressif `esp` Rust toolchain, installed using `espup`.
With that toolchain available:

```sh
cargo +esp check --release --no-default-features --features esp32s3 --target xtensa-esp32s3-none-elf -Zbuild-std=core,alloc
cargo +esp build --release --no-default-features --features esp32s3 --target xtensa-esp32s3-none-elf -Zbuild-std=core,alloc
```

The ESP radio dependency needs `alloc` as well as `core`. For another chip,
change both the chip feature and target; RISC-V ESP chips use a RISC-V target,
not an Xtensa target.

In a firmware application's `Cargo.toml`, select the feature explicitly:

```toml
[dependencies]
agnostic_mmc = { path = "../v1", default-features = false, features = ["esp32s3"] }
```

The application owns its target/toolchain configuration and runtime setup.
These commands build a library, not a flashable firmware image, and do not
verify hardware behavior. The internal ESP HTTP-server implementation is still
unfinished, including a `todo!()` in its setup function.
