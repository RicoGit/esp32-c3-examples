//! Writes servo sg90, uses ledc.
//! Servo data cable connected with gpio6.
//!
//! `cargo run --example ledc_servo`

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
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
        .frequency(50.Hz().into())
        .resolution(Resolution::Bits10)
        .speed_mode(SpeedMode::LowSpeed);
    let timer_driver = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;
    let mut driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio6
    )?;

    let max_duty = driver.get_max_duty(); // 1023, see Resolution::Bits10 above
    log::info!("Max duty {}", max_duty);

    let mut red_led_pin = PinDriver::output(peripherals.pins.gpio3).unwrap();
    let mut green_led_pin = PinDriver::output(peripherals.pins.gpio4).unwrap();
    let mut blue_led_pin = PinDriver::output(peripherals.pins.gpio5).unwrap();
    loop {
        driver.set_duty(1)?;
        log::info!("Max duty {}", driver.get_duty());
        red_led_pin.set_high()?;
        FreeRtos::delay_ms(5000);
        red_led_pin.set_low()?;

        driver.set_duty(512)?;
        log::info!("Max duty {}", driver.get_duty());
        green_led_pin.set_high()?;
        FreeRtos::delay_ms(5000);
        green_led_pin.set_low()?;

        driver.set_duty(1023)?;
        log::info!("Max duty {}", driver.get_duty());
        blue_led_pin.set_high()?;
        FreeRtos::delay_ms(5000);
        blue_led_pin.set_low();
    }

}
