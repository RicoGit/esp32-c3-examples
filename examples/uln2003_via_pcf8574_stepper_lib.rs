//! Stepper motor with ULN2003 driver via pin expander (PCF8574) with stepper lib.
//!
//! io5 - sda
//! io6 - scl
//!
//! `cargo run --example uln2003_via_pcf8574_stepper_lib`

use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;
use pcf857x::pcf8574::Parts;
use pcf857x::{OutputPin, Pcf8574, SlaveAddr};

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
    let mut expander: Pcf8574<I2cDriver> =
        Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));

    let Parts { p4, p5, p6, p7, .. } = expander.split();

    // todo
}

struct Motor<P1, P2, P3, P4> {
    step: u32,
    int1: P1,
    int2: P2,
    int3: P3,
    int4: P4,
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> Motor<P1, P2, P3, P4> {
    pub fn new(mut pin1: P1, mut pin2: P2, mut pin3: P3, mut pin4: P4) -> Self {
        let _ = pin1.set_low();
        let _ = pin2.set_low();
        let _ = pin3.set_low();
        let _ = pin4.set_low();
        Motor {
            step: 0,
            int1: pin1,
            int2: pin2,
            int3: pin3,
            int4: pin4,
        }
    }

    fn step_forward(&mut self) {
        self.do_step();
        if self.step == 7 {
            self.step = 0;
        } else {
            self.step += 1;
        }
    }

    fn step_back(&mut self) {
        self.do_step();
        if self.step == 0 {
            self.step = 7;
        } else {
            self.step -= 1;
        }
    }

    fn do_step(&mut self) {
        match self.step {
            0 => {
                self.int1.set_low();
                self.int2.set_low();
                self.int3.set_low();
                self.int4.set_high();
            }
            1 => {
                self.int1.set_low();
                self.int2.set_low();
                self.int3.set_high();
                self.int4.set_high();
            }
            2 => {
                self.int1.set_low();
                self.int2.set_low();
                self.int3.set_high();
                self.int4.set_low();
            }
            3 => {
                self.int1.set_low();
                self.int2.set_high();
                self.int3.set_high();
                self.int4.set_low();
            }
            4 => {
                self.int1.set_low();
                self.int2.set_high();
                self.int3.set_low();
                self.int4.set_low();
            }
            5 => {
                self.int1.set_high();
                self.int2.set_high();
                self.int3.set_low();
                self.int4.set_low();
            }
            6 => {
                self.int1.set_high();
                self.int2.set_low();
                self.int3.set_low();
                self.int4.set_low();
            }
            7 => {
                self.int1.set_high();
                self.int2.set_low();
                self.int3.set_low();
                self.int4.set_high();
            }
            _ => {
                self.int1.set_low();
                self.int2.set_low();
                self.int3.set_low();
                self.int4.set_low();
            }
        }
    }
}
