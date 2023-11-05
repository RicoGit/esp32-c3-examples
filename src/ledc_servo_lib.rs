//! This is a small lib for controlling servo using LEDC.

use std::marker::PhantomData;
use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::ledc;
use esp_idf_svc::hal::ledc::{LedcChannel, LedcTimer};
use esp_idf_svc::hal::peripheral::{Peripheral};
use esp_idf_svc::hal::prelude::{FromValueType, Hertz};
use esp_idf_svc::sys::EspError;

pub struct ServoBuilder {
}

impl ServoBuilder {
}

pub struct ServoConfig {
    /// Max angle that servo can't be turned, mostly 180, 360.
    max_angle: u16,
    /// Resolution in bits.
    resolution: ledc::Resolution,
    /// ESP32 supports High Speed Mode.
    /// ESP32S2, ESP32S3, ESP32C2 and ESP32C3 supports Low Speed Mode.
    speed_mode: ledc::SpeedMode,
    /// What frequency expect servo (ex. 50Hz for SG90)
    frequency: Hertz,

}


impl ServoConfig {
    /// Config for [SG90](https://components101.com/motors/servo-motor-basics-pinout-datasheet).
    pub fn sg90(speed_mode: ledc::SpeedMode) -> Self {
        ServoConfig {
            max_angle: 180,
            resolution: ledc::Resolution::Bits10,
            speed_mode,
            frequency: 50.Hz().into(),
        }
    }

    /// Config for [SG90S](https://components101.com/motors/mg90s-metal-gear-servo-motor).
    fn sg90s(speed_mode: ledc::SpeedMode)  -> Self {
        Self::sg90(speed_mode)
    }
}


pub struct Servo<'d> {
    ledc_driver: ledc::LedcDriver<'d>,
    _p: PhantomData<&'d mut ()>,
}

impl<'d> Servo<'d> {

    pub fn new<T: LedcTimer, C: LedcChannel, P: OutputPin>(
        config: ServoConfig,
        timer: impl Peripheral<P = T> + 'd,
        channel: impl Peripheral<P = C> + 'd,
        pin: impl Peripheral<P = P> + 'd
    ) -> Result<Servo<'d>, EspError> {

       let timer_config= ledc::config::TimerConfig::default()
           .resolution(config.resolution)
           .speed_mode(config.speed_mode)
           .frequency(config.frequency);

        let timer_driver = ledc::LedcTimerDriver::new(timer, &timer_config)?;

        let ledc_driver = ledc::LedcDriver::new(
            channel,
            timer_driver,
            pin
        )?;

        Ok(Servo {
            ledc_driver,
            _p: PhantomData,
        })
    }
}
