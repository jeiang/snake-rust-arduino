`Snake using Rust on Arduino?`
==================

Snake made using Rust on an Arduino Mega 2560. Likely will work on the following: 
 - Arduino Leonardo
 - Arduino Nano
 - Arduino Uno
 - SparkFun ProMicro
 - Adafruit Trinket
 - Adafruit Trinket Pro
with some modification.

## Requirements
1. Compatible Arduino (see [avr-hal](https://github.com/Rahix/avr-hal))
2. Toolchain & ravedude (see [Quickstart](https://github.com/Rahix/avr-hal#quickstart))
3. MAX7219 8x8 Matrix LED Display
4. Analog Joystick with pressbutton
5. Wires, power supply (the Arduino Mega 2560 does not provide enough current to power the display), etc.

## Usage
Connect the Arduino (if not an Arduino Mega 2560, ensure necessary modifications are made) via USB and run
```bash
cargo run --release
```
Assuming no problems with detection, the serial output should be a reflection of the game state. If you do not see output on the Arduino, ensure that the serial output shows a the "game" and ensure connections are made correctly. 

## License
Licensed under either of
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
