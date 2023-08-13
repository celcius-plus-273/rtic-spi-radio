# RUST RFM95W SPI RADIO

This project contains a working example of the sx127x_lora driver with a Teensy 4.0 or 4.1 on Rust. It uses the RTIC 2.0 framework.

Checkout the [examples](https://github.com/celcius-plus-273/rtic-spi-radio/tree/main/examples) directory for more information on Usage.

## Dependencies
1. You'll need to install [nightly rust](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml).
2. Follow the instructions under the [Dependencies](https://github.com/mciantyre/teensy4-rs) section and download all required dependencies
3. Connect a Teensy 4.0 or 4.1 board via USB to your PC
4. Use the cargo run command with the appropriate target. For example:

```
cargo run --release --target thumbv7em-none-eabihf
```

5. Don't forget to press the pushbutton on the Teensy board to enter it's programming mode
