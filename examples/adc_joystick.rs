//! Read analog signals from simple joystick ([like this one](https://components101.com/modules/joystick-module)).
//!
//! `cargo run --example adc_joystick`

use esp_idf_svc::hal::adc::{ADC1, AdcChannelDriver, AdcDriver, attenuation};
use esp_idf_svc::hal::adc::config::Config;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::*;

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();


    let mut adc = AdcDriver::new(peripherals.adc1, &Config::new().calibration(true))?;

    // configuring pin to analog read, you can regulate the adc input voltage range depending on your need
    // for this example we use the attenuation of 11db which sets the input voltage range to around 0-3.6V
    let mut adc_pin_x: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio0)?;

    let mut adc_pin_y: AdcChannelDriver<{ attenuation::DB_11 }, _> =
        AdcChannelDriver::new(peripherals.pins.gpio1)?;

    loop {
        // you can change the sleep duration depending on how often you want to sample
        FreeRtos::delay_ms(20);
        println!("X: {}, Y: {}", adc.read(&mut adc_pin_x)?, adc.read(&mut adc_pin_y)?);
    }

}
