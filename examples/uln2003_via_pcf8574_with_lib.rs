//! Stepper motor with ULN2003 driver via pin expander (PCF8574) with uln2003 lib.
//!
//! io5 - sda
//! io6 - scl
//!
//! `cargo run --example uln2003_via_pcf8574_with_lib`

use esp_idf_svc::hal::delay::{Delay, Ets, FreeRtos};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;

use pcf857x::{InputPin, OutputPin, Pcf8574, SlaveAddr};
use pcf857x::pcf8574::Parts;
use uln2003::StepperMotor;

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

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let mut expander: Pcf8574<I2cDriver> = Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));

    let Parts { p4, p5, p6, p7, .. } = expander.split();

    let mut motor = uln2003::ULN2003::new(p7, p6, p5, p4, Some(Delay::new_default()));

    loop {
        log::info!("move forward");
        let _ = motor.step_for(2048, 2);

        log::info!("move backward");
        let _ = motor.step_for(2048, 2);
    }
}
