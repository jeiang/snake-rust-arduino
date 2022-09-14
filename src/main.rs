#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(panic_internals)]

mod analog_stick;
mod game;
mod matrix_display;
mod panic_handler;
mod rand;
mod spelling;

// use panic_halt as _;
use arduino_hal::{delay_ms, prelude::*};
use game::{direction::Direction, Command, Game};
use matrix_display::MAX7219;
use rand::RandomGenerator;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Starting...").void_unwrap();

    let mut adc = arduino_hal::adc::Adc::new(dp.ADC, Default::default());
    let x_pin = pins.a0.into_analog_input(&mut adc).into_channel();
    let y_pin = pins.a1.into_analog_input(&mut adc).into_channel();
    let switch = pins.a2.into_pull_up_input().downgrade();
    let mut stick = analog_stick::AnalogStick::new(&mut adc, x_pin, y_pin, switch);

    let clk = pins.d22.into_output();
    let cs = pins.d23.into_output();
    let data = pins.d24.into_output();

    let mut display = MAX7219::new(data, cs, clk);
    display.power_on();
    display.set_intensity(1);
    let mut game = Game::new(RandomGenerator::new(0xDEADBEEF));
    let mut direction = Direction::Right;

    loop {
        ufmt::uwriteln!(&mut serial, "Reading...").void_unwrap();
        let cmd = {
            let mut pressed = false;
            for _ in 0..10 {
                delay_ms(10);
                let reading = stick.get_reading();
                pressed |= reading.is_pressed;
                if let Some(dir) = reading.to_direction() {
                    direction = dir;
                }
            }
            if pressed {
                Command::Reset
            } else {
                Command::Move(direction)
            }
        };
        ufmt::uwriteln!(serial, "Command: {:?}", direction).void_unwrap();

        ufmt::uwriteln!(&mut serial, "Stepping...").void_unwrap();
        match game.step(cmd) {
            game::GameResult::Continue => {
                ufmt::uwriteln!(&mut serial, "Moving...").void_unwrap();
            }
            game::GameResult::Died => {
                ufmt::uwriteln!(&mut serial, "Died...").void_unwrap();
                spelling::print_lose(&mut display);
                display.clear_display();
            }
            game::GameResult::Won => {
                ufmt::uwriteln!(&mut serial, "Won...").void_unwrap();
                spelling::print_win(&mut display);
                display.clear_display();
            }
            game::GameResult::Restarting => {
                ufmt::uwriteln!(&mut serial, "Restarting...").void_unwrap();
                for _ in 0..2 {
                    display.flash(true);
                    delay_ms(500);
                    display.flash(false);
                    delay_ms(500);
                }
                display.clear_display();
            }
        }

        // TODO: print the screen map
        // display.clear_display();
        ufmt::uwriteln!(&mut serial, "Printing...").void_unwrap();
        ufmt::uwriteln!(&mut serial, "Game:\n{}", game).void_unwrap();

        let last_snake_pos = game.last_snake_tail();
        display.write_pos(last_snake_pos.x(), last_snake_pos.y(), false);
        let apple = game.apple();
        display.write_pos(apple.x(), apple.y(), true);
        for pos in game.iter_snake() {
            display.write_pos(pos.x(), pos.y(), true);
        }
    }
}
