use arduino_hal::{
    adc::Channel,
    port::{
        mode::{Input, PullUp},
        Pin,
    },
    Adc,
};
use ufmt::{derive::uDebug, uDisplay, uwrite};

use crate::game::direction::Direction;

const MAX_ADC_READING: u16 = 0b11_1111_1111;
// NOTE: Based on experimentation, center results in ~500 as the reading
const HALF_MAX_ADC_READING: i16 = 500;
const QUARTER_MAX_ADC_READING: i16 = HALF_MAX_ADC_READING >> 1;

pub struct AnalogStick<'adc> {
    adc: &'adc mut Adc,
    x_pin: Channel,
    y_pin: Channel,
    switch: Pin<Input<PullUp>>,
}

#[derive(Default, uDebug)]
pub struct AnalogReading {
    pub x: i16,
    pub y: i16,
    pub is_pressed: bool,
}

impl AnalogReading {
    pub fn to_direction(&self) -> Option<Direction> {
        if self.x > QUARTER_MAX_ADC_READING {
            Some(Direction::Left)
        } else if self.x < -QUARTER_MAX_ADC_READING {
            Some(Direction::Right)
        } else if self.y > QUARTER_MAX_ADC_READING {
            Some(Direction::Up)
        } else if self.y < -QUARTER_MAX_ADC_READING {
            Some(Direction::Down)
        } else {
            None
        }
    }
}

impl uDisplay for AnalogReading {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let pressed_str = if self.is_pressed {
            "Button Pressed"
        } else {
            "Button Not Pressed"
        };
        uwrite!(f, "X: {}, Y: {}, {}", self.x, self.y, pressed_str)
    }
}

impl<'adc> AnalogStick<'adc> {
    pub fn new(
        adc: &'adc mut Adc,
        x_pin: Channel,
        y_pin: Channel,
        switch: Pin<Input<PullUp>>,
    ) -> Self {
        Self {
            adc,
            x_pin,
            y_pin,
            switch,
        }
    }

    pub fn get_reading(&mut self) -> AnalogReading {
        let x = self.adc.read_blocking(&self.x_pin) as i16;
        let x = x - HALF_MAX_ADC_READING;
        let y = self.adc.read_blocking(&self.y_pin) as i16;
        let y = y - HALF_MAX_ADC_READING;
        let is_pressed = self.switch.is_low();

        AnalogReading { x, y, is_pressed }
    }
}
