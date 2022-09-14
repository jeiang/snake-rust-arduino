//! TAKEN FROM https://github.com/maikelwever/max7219/blob/master/src/lib.rs
//! Modified to be compatible with newer version of embedded-hal and to be customized to this
//! crate.
//!
//! ORIGINAL:
//!
//! A platform agnostic driver to interface with the MAX7219 (LED matrix display driver)
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1

#![deny(unsafe_code)]
//#![deny(warnings)]

use embedded_hal::digital::v2::OutputPin;

pub enum Command {
    Noop = 0x00,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Power = 0x0C,
    DisplayTest = 0x0F,
}

pub enum DecodeMode {
    NoDecode = 0x00,
    CodeBDigit0 = 0x01,
    CodeBDigits3_0 = 0x0F,
    CodeBDigits7_0 = 0xFF,
}

pub struct MAX7219<DATA, CS, CLK> {
    data: DATA,
    cs: CS,
    clk: CLK,
    buffer: [u8; 8],
}

impl<DATA, CS, CLK> MAX7219<DATA, CS, CLK>
where
    DATA: OutputPin,
    CS: OutputPin,
    CLK: OutputPin,
{
    pub fn new(data: DATA, cs: CS, clk: CLK) -> Self {
        let mut max7219 = MAX7219 {
            data,
            cs,
            clk,
            buffer: [0; 8],
        };

        max7219.init();
        max7219
    }

    pub fn init(&mut self) {
        self.write_command(Command::DisplayTest);
        self.write_data(Command::ScanLimit, 0x07);
        self.set_decode_mode(DecodeMode::NoDecode);
        self.clear_display();
        self.power_off();
    }

    pub fn set_decode_mode(&mut self, mode: DecodeMode) {
        self.write_data(Command::DecodeMode, mode as u8);
    }

    pub fn power_on(&mut self) {
        self.write_data(Command::Power, 0x01);
    }

    pub fn power_off(&mut self) {
        self.write_data(Command::Power, 0x00);
    }

    pub fn write_command(&mut self, command: Command) {
        self.write_data(command, 0x00);
    }

    pub fn write_data(&mut self, command: Command, data: u8) {
        self.write_raw(command as u8, data);
    }

    pub fn write_pos(&mut self, x: u8, y: u8, state: bool) {
        let x = x & 0x07;
        let y = y & 0x07;
        let row = self.buffer[7 - x as usize];
        let row = if state {
            row | (1 << y)
        } else {
            row & !(1 << y)
        };
        self.write_row(x, row);
    }

    pub fn write_row(&mut self, x: u8, row: u8) {
        let x = x & 0x07;
        self.write_raw(8 - x, row);
    }

    pub fn write_raw(&mut self, header: u8, data: u8) {
        // Save the "pixel state" to the internal buffer
        if 0 < header && header < 9 {
            self.buffer[header as usize - 1] = data;
        }

        let data = (data as u16) | ((header as u16) << 8);
        
        _ = self.cs.set_low();
        self.shift_out(data);
        _ = self.cs.set_high();
    }

    pub fn set_intensity(&mut self, intensity: u8) {
        self.write_data(Command::Intensity, intensity);
    }

    fn shift_out(&mut self, value: u16) {
        for i in 0..16 {
            if value & (1 << (15 - i)) > 0 {
                _ = self.data.set_high();
            } else {
                _ = self.data.set_low();
            }

            _ = self.clk.set_high();
            _ = self.clk.set_low();
        }
    }

    pub fn clear_display(&mut self) {
        for i in 1..9 {
            self.write_raw(i, 0x00);
        }
    }

    pub fn test(&mut self, is_on: bool) {
        if is_on {
            self.write_raw(0x01, 0x01);
        } else {
            self.write_raw(0x01, 0x00);
        }
    }

    pub fn flash(&mut self, on: bool) {
        let state = if on {
            0xFF
        } else {
            0x00
        };
        for i in 0..8 {
            self.write_row(i, state);
        }
    }
}
