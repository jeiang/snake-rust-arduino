use arduino_hal::delay_ms;

use crate::matrix_display::MAX7219;

const WIN: [u8; 24] = [
    0b11111111, 0b00000010, 0b00000100, 0b00001000, 0b00001000, 0b00000100, 0b00000010, 0b11111111,
    0b00000000, 0b00000000, 0b00100010, 0b10111110, 0b10111110, 0b00000010, 0b00000000, 0b00000000,
    0b00111110, 0b00111110, 0b00100000, 0b00100000, 0b00100000, 0b00111110, 0b00011110, 0b00000000,
];

const LOSE: [u8; 32] = [
    0b11111110, 0b11111110, 0b00000010, 0b00000010, 0b00000010, 0b00000010, 0b00000010, 0b00000000,
    0b00011100, 0b00111110, 0b00100010, 0b00100010, 0b00100010, 0b00111110, 0b00011100, 0b00000000,
    0b00010010, 0b00111010, 0b00101010, 0b00101010, 0b00101010, 0b00101110, 0b00100100, 0b00000000,
    0b00011100, 0b00111110, 0b00101010, 0b00101010, 0b00101010, 0b00111010, 0b00011000, 0b00000000,
];

pub fn print_lose<DATA, CS, CLK>(display: &mut MAX7219<DATA, CS, CLK>)
where
    DATA: embedded_hal::digital::v2::OutputPin,
    CS: embedded_hal::digital::v2::OutputPin,
    CLK: embedded_hal::digital::v2::OutputPin,
{
    display.clear_display();
    for i in 0..24 {
        for j in 0..8 {
            display.write_row(j, LOSE[(i + j) as usize]);
        }
        if i == 0 || i == 24 {
            delay_ms(500);
        } else {
            delay_ms(200);
        }
    }
    delay_ms(1000);
}

pub fn print_win<DATA, CS, CLK>(display: &mut MAX7219<DATA, CS, CLK>)
where
    DATA: embedded_hal::digital::v2::OutputPin,
    CS: embedded_hal::digital::v2::OutputPin,
    CLK: embedded_hal::digital::v2::OutputPin,
{
    display.clear_display();
    for i in 0..16 {
        for j in 0..8 {
            display.write_row(j, WIN[(i + j) as usize]);
        }
        if i == 0 || i == 24 {
            delay_ms(500);
        } else {
            delay_ms(200);
        }
    }
    delay_ms(1000);
}
