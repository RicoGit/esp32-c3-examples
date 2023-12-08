//! Finds all ds18b20 sensors and reads temperature.
//! All sensors are connected to gpio6.
//!
//! `cargo run --example ds18b20_simple`

use ds18b20::Ds18b20;
use esp_idf_svc::hal::delay::Delay;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::*;
use one_wire_bus::Address;

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let pin6 = PinDriver::input_output(peripherals.pins.gpio6)?;
    let mut one_wire_bus = one_wire_bus::OneWire::new(pin6).expect("one wire bus error");
    let mut delay = Delay::new(10_000);

    let devices = one_wire_bus.devices(false, &mut delay).collect::<Vec<_>>();
    log::info!("Found devices: {:?}", devices);

    let devices = devices
        .into_iter()
        .map(|d| Ds18b20::new::<()>(Address(d.unwrap().0)).unwrap())
        .collect::<Vec<_>>();

    loop {
        ds18b20::start_simultaneous_temp_measurement(&mut one_wire_bus, &mut delay).unwrap();
        for device in &devices {
            let result = device
                .read_data(&mut one_wire_bus, &mut delay)
                .expect("temp read");
            log::info!("[{:?}] Temp is: {:?}", device.address(), result);
        }
        delay.delay_ms(2000);
    }
}
