use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;

// led1 - gpio3, gpio4, gpio5 (rgb)
// led2 - gpio18
// led3 - gpio19
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let per = Peripherals::take().unwrap();
    let mut red_led_pin = PinDriver::output(per.pins.gpio3).unwrap();
    let mut green_led_pin = PinDriver::output(per.pins.gpio4).unwrap();
    let mut blue_led_pin = PinDriver::output(per.pins.gpio5).unwrap();

    let mut led2_pin = PinDriver::output(per.pins.gpio18).unwrap();
    let mut led3_pin = PinDriver::output(per.pins.gpio19).unwrap();

    let mut tmp = false;
    loop {
        log::info!("start loop");
        
        if tmp {
            led2_pin.set_low().unwrap();
            led3_pin.set_high().unwrap()
        } else {
            led2_pin.set_high().unwrap();
            led3_pin.set_low().unwrap()
        }
        tmp = !tmp;
        blue_led_pin.set_low().unwrap();
        red_led_pin.set_high().unwrap();
        FreeRtos::delay_ms(500);

        red_led_pin.set_low().unwrap();
        green_led_pin.set_high().unwrap();
        FreeRtos::delay_ms(500);

        green_led_pin.set_low().unwrap();
        blue_led_pin.set_high().unwrap();
        FreeRtos::delay_ms(500);

    }

}
