# TEENSY4-RTIC-TEMPLATE

Template for a teensy 4.0/4.1 rust RTIC embedded project

This template uses the 2.0 version of RTIC

## rtic branch usage
### Dependencies
1. You'll need to install [nightly rust](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml).
2. Follow the instructions under the [Dependencies](https://github.com/mciantyre/teensy4-rs) section and download all required dependencies
3. Connect a Teensy 4.0 or 4.1 board via USB to your PC
4. Use the cargo run command with the appropriate target. For example:
```
cargo run --release --target thumbv7em-none-eabihf
```
5. Don't forget to press the pushbutton on the Teensy board to enter it's programming mode
