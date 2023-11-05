use esp_idf_svc::hal::ledc;
use esp_idf_svc::hal::prelude::*;

mod ledc_servo_lib;
use ledc_servo_lib::{ServoConfig, Servo};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    println!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let config = ServoConfig::sg90(ledc::SpeedMode::LowSpeed);
}
