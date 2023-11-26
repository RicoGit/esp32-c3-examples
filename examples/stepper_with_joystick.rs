//! Joystick controls 2 `28byj-48` motors (driver: `ULN2003`, poert expander: `PCF8574`).
//!
//! * io5 - sda of PCF8574
//! * io6 - scl of PCF8574
//!
//! `cargo run --example stepper_with_joystick

use std::ops::RangeInclusive;
use esp_idf_svc::hal::adc::{AdcChannelDriver, AdcDriver, attenuation};
use esp_idf_svc::hal::adc::config::Config;

use esp_idf_svc::hal::delay::{Delay};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;

use pcf857x::{Pcf8574, SlaveAddr};
use pcf857x::pcf8574::Parts;
use uln2003::{Direction, StepperMotor};

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    println!("start");

    let peripherals = Peripherals::take().unwrap();

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    // joystick
    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;
    let mut adc_pin_x: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio0)?;
    let mut adc_pin_y: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio1)?;

    // port expander (PCF8574)
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let expander: Pcf8574<I2cDriver> = Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));
    let Parts { p7, p6, p5, p4, p3, p2, p1, p0 } = expander.split();
    
    // motors
    let delay = Delay::new(10_000);
    let mut motor1 = uln2003::ULN2003::<_,_,_,_, u32, _>::new(p7, p6, p5, p4, Some(delay.clone()));
    let mut motor2 = uln2003::ULN2003::<_,_,_,_, u32, _>::new(p3, p2, p1, p0, Some(delay));

    let mut motor1_last_step_elapsed = 0;
    let mut motor2_last_step_elapsed = 0;
    loop {
        match Cmd::from_joystick_value(adc.read(&mut adc_pin_x)?) {
            Cmd::Stop => { motor1.stop().unwrap(); }
            Cmd::Forward { delay_ms } => {
                if delay_ms < motor1_last_step_elapsed {
                    motor1.set_direction(Direction::Normal);
                    motor1.step().unwrap();
                    motor1_last_step_elapsed = 0;
                };
            }
            Cmd::Backward { delay_ms } => {
                if delay_ms < motor1_last_step_elapsed {
                    motor1.set_direction(Direction::Reverse);
                    motor1.step().unwrap();
                    motor1_last_step_elapsed = 0;
                };
            }
        }

        match Cmd::from_joystick_value(adc.read(&mut adc_pin_y)?) {
            Cmd::Stop => { motor2.stop().unwrap(); }
            Cmd::Forward { delay_ms } => {
                if delay_ms < motor2_last_step_elapsed {
                    motor2.set_direction(Direction::Normal);
                    motor2.step().unwrap();
                    motor2_last_step_elapsed = 0;
                };
            }
            Cmd::Backward { delay_ms } => {
                if delay_ms < motor2_last_step_elapsed {
                    motor2.set_direction(Direction::Reverse);
                    motor2.step().unwrap();
                    motor2_last_step_elapsed = 0;
                };
            }
        }
        delay.delay_ms(MIN_DELAY_MS);
        motor1_last_step_elapsed += MIN_DELAY_MS;
        motor2_last_step_elapsed += MIN_DELAY_MS;
    }
}


const MIN_DELAY_MS: u32 = 1;
const MAX_DELAY_MS: u32 = 20;

const JOYSTICK_MIN: u32 = 0;
const JOYSTICK_CENTER: u32 = 1620;
const JOYSTICK_MAX: u32 = 2081;
const JOYSTICK_THRESHOLD: u32 = 50;
const JOYSTICK_CENTER_RANGE: RangeInclusive<u32> = JOYSTICK_CENTER - JOYSTICK_THRESHOLD..=JOYSTICK_CENTER + JOYSTICK_THRESHOLD;
const JOYSTICK_START_RANGE: RangeInclusive<u32> = JOYSTICK_MIN..=JOYSTICK_CENTER - JOYSTICK_THRESHOLD;
const JOYSTICK_END_RANGE: RangeInclusive<u32> = JOYSTICK_CENTER + JOYSTICK_THRESHOLD..=JOYSTICK_MAX;

enum Cmd {
    Stop,
    Forward {
        /// Delay between steps in milliseconds.
        delay_ms: u32
    },
    Backward {
        /// Delay between steps in milliseconds.
        delay_ms: u32
    },
}

impl Cmd {
    fn from_joystick_value(joystick_value: u16) -> Self {
        match joystick_value as u32 {
            val if JOYSTICK_CENTER_RANGE.contains(&val) => Cmd::Stop,
            val if JOYSTICK_START_RANGE.contains(&val) => {
                let delay_ms = Self::value_to_delay(val);
                Cmd::Forward { delay_ms }
            }
            val if JOYSTICK_END_RANGE.contains(&val) => {
                let delay_ms = Self::value_to_delay(val);
                Cmd::Backward { delay_ms }
            }
            _ => panic!("Joystick value is out of range: {}", joystick_value)
        }
    }

    /// Converts joystick value to the delay in milliseconds.
    /// The more the joystick is deflected, the faster the motor moves.
    fn value_to_delay(joystick_value: u32) -> u32 {
        joystick_value * MAX_DELAY_MS / JOYSTICK_MAX
    }
}
