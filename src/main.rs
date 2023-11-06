use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::ledc;
use esp_idf_svc::hal::prelude::*;

use eyre::Result;
mod ledc_servo_lib;
use ledc_servo_lib::{Servo, ServoConfig};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    println!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let config = ServoConfig::sg90(ledc::SpeedMode::LowSpeed);
    let mut servo = Servo::new(
        config,
        peripherals.ledc.timer0,
        peripherals.ledc.channel0,
        peripherals.pins.gpio6,
    )?;

    loop {
        servo.set_angle(0.0)?;
        log::info!("current angle {}", servo.get_angle());
        FreeRtos::delay_ms(5000);

        servo.set_angle(90.0)?;
        log::info!("current angle {}", servo.get_angle());
        FreeRtos::delay_ms(5000);

        servo.set_angle(180.0)?;
        log::info!("current angle {}", servo.get_angle());
        FreeRtos::delay_ms(5000);
    }
}
