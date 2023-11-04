//! Smoothly blinks board led (gpio18) via PWM signal.
//!
//! `cargo run --example ledc`

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::ledc;
use esp_idf_svc::hal::ledc::{LedcDriver, LedcTimerDriver, Resolution, SpeedMode};
use esp_idf_svc::hal::prelude::*;

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let timer_config = ledc::config::TimerConfig::default()
        .frequency(25.kHz().into())
        .resolution(Resolution::Bits10)
        .speed_mode(SpeedMode::LowSpeed);
    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;
    let mut driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio18
    )?;

    let max_duty = driver.get_max_duty(); // 1023, see Resolution::Bits10 above
    log::info!("Max duty {}", max_duty);

    let mut counter: i32 = 0;
    let mut step: i32 = 20;
    loop {
        if counter as u32 >= max_duty {
            step = -20;
        }

        if counter <= 0 {
            step = 20;
        }

        counter += step;

        let _ = driver.set_duty(counter as u32)?;
        FreeRtos::delay_ms(20);
    }

}
