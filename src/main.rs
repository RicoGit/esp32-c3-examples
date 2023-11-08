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

    let config = ServoConfig {
        pulse_width_ns: 500..2400,
        ..ServoConfig::sg90(ledc::SpeedMode::LowSpeed)
    };
    let mut servo = Servo::new(
        config,
        peripherals.ledc.timer0,
        peripherals.ledc.channel0,
        peripherals.pins.gpio6,
    )?;

    let mut ang = 0.0;
    loop {
        // 0 - 90 - 180
        servo.set_angle(0.0)?;
        println!(
            "current angle {} {}",
            servo.get_angle(),
            servo.ledc_driver.get_duty()
        );
        FreeRtos::delay_ms(2000);

        servo.set_angle(40.0)?;
        println!(
            "current angle {} {}",
            servo.get_angle(),
            servo.ledc_driver.get_duty()
        );
        FreeRtos::delay_ms(2000);

        servo.set_angle(70.0)?;
        println!(
            "current angle {} {}",
            servo.get_angle(),
            servo.ledc_driver.get_duty()
        );
        FreeRtos::delay_ms(2000);

        // swing
        loop {
            servo.set_angle(ang)?;
            FreeRtos::delay_ms(100);
            println!("current angle {}=={} {}", ang, servo.get_angle(), servo.ledc_driver.get_duty());
            ang += 1.0;

            if ang == 70.0 {
                ang = 0.0;
                break;
            }
        }
    }
}
