//! Joystick controls 2 `28byj-48` motors (driver: `ULN2003`, poert expander: `PCF8574`).
//!
//! * io5 - sda of PCF8574
//! * io6 - scl of PCF8574
//!
//! `cargo run --example stepper_with_joystick

use esp_idf_svc::hal::adc::config::Config;
use esp_idf_svc::hal::adc::{attenuation, AdcChannelDriver, AdcDriver};
use std::ops::RangeInclusive;

use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;

use pcf857x::pcf8574::Parts;
use pcf857x::{Pcf8574, SlaveAddr};
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

    log::info!("init port expander");
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let expander: Pcf8574<I2cDriver> = Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));
    let Parts {
        p7,
        p6,
        p5,
        p4,
        p3,
        p2,
        p1,
        p0,
    } = expander.split();

    log::info!("init motor");
    let delay = Delay::new(10_000);
    let mut motor1 =
        uln2003::ULN2003::<_, _, _, _, u32, _>::new(p7, p6, p5, p4, Some(delay.clone()));
    let mut motor2 = uln2003::ULN2003::<_, _, _, _, u32, _>::new(p3, p2, p1, p0, Some(delay));

    let mut motor1_last_step_elapsed = 0;
    let mut motor2_last_step_elapsed = 0;

    log::info!("start loop");
    loop {
        match Cmd::from_joystick_value(adc.read(&mut adc_pin_x)?) {
            Cmd::Stop => {
                motor1.stop().unwrap();
            }
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
            Cmd::Stop => {
                motor2.stop().unwrap();
            }
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
        delay.delay_us(100);
        motor1_last_step_elapsed += MIN_DELAY_MS;
        motor2_last_step_elapsed += MIN_DELAY_MS;
    }
}

const MIN_DELAY_MS: u32 = 1;
const MAX_DELAY_MS: u32 = 20;

const JOYSTICK_MIN: u32 = 1;
const JOYSTICK_CENTER: u32 = 1650;
const JOYSTICK_MAX: u32 = 2801;
const JOYSTICK_THRESHOLD: u32 = 30;
const JOYSTICK_CENTER_RANGE: RangeInclusive<u32> =
    JOYSTICK_CENTER - JOYSTICK_THRESHOLD..=JOYSTICK_CENTER + JOYSTICK_THRESHOLD;

const RATIO: f64 = MAX_DELAY_MS as f64 / JOYSTICK_MAX as f64;

#[derive(Debug)]
enum Cmd {
    Stop,
    Forward {
        /// Delay between steps in milliseconds.
        delay_ms: u32,
    },
    Backward {
        /// Delay between steps in milliseconds.
        delay_ms: u32,
    },
}

impl Cmd {
    fn from_joystick_value(joystick_value: u16) -> Self {
        let res = match joystick_value as u32 {
            val if JOYSTICK_CENTER_RANGE.contains(&val) => Cmd::Stop,
            val if val <= JOYSTICK_CENTER => {
                let delay_ms = Self::value_to_delay(val, false);
                Cmd::Forward { delay_ms }
            }
            val if val > JOYSTICK_CENTER => {
                let delay_ms = Self::value_to_delay(val, true);
                Cmd::Backward { delay_ms }
            }
            _ => panic!("Joystick value is out of range: {}", joystick_value),
        };
        log::debug!("cmd =  {res:?} for {joystick_value}");
        res
    }

    /// Converts joystick value to the delay in milliseconds.
    /// The more the joystick is deflected, the faster the motor moves.
    fn value_to_delay(joystick_value: u32, invert: bool) -> u32 {
        let value = joystick_value.min(JOYSTICK_MAX).max(JOYSTICK_MIN) as f64;
        if invert {
            (MAX_DELAY_MS - (value * RATIO) as u32).max(MIN_DELAY_MS)
        } else {
            ((value * RATIO) as u32).max(MIN_DELAY_MS)
        }
    }
}
