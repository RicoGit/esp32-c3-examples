//! Stepper motor with ULN2003 driver via pin expander (PCF8574)
//!
//! # PCF8574
//!
//! io5 - sda
//! io6 - scl
//!
//!
//! # 28BYJ-48 steps
//!
//!  * In1 In2 In3 In4
//!  *  1   0   1   0
//!  *  0   1   1   0
//!  *  0   1   0   1
//!  *  1   0   0   1
//!
//! (inspired by https://github.com/arduino-libraries/Stepper/blob/master/src/Stepper.cpp)
//! `cargo run --example uln2003_via_pcf8574`

use esp_idf_svc::hal::delay::{Ets};
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::prelude::*;

use pcf857x::{OutputPin, Pcf8574, SlaveAddr};
use pcf857x::pcf8574::Parts;

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    println!("start uln2003_via_pcf8574");

    let peripherals = Peripherals::take().unwrap();

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio6;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let mut expander: Pcf8574<I2cDriver> = Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));

    let Parts { p4, p5, p6, p7, .. } = expander.split();
    let mut motor = Motor::new(p4, p5, p6, p7);

    loop {
        log::info!("move forward");
        for _ in 0..4096 {
            motor.step_forward();
            // FreeRtos::delay_ms(20)
            Ets::delay_ms(2);
        }

        log::info!("move backward");
        for _ in 0..4096 {
            motor.step_back();
            // FreeRtos::delay_ms(20)
            Ets::delay_ms(2);
        }
    }
}

struct Motor<P1, P2, P3, P4> {
    step: u32,
    int1: P1,
    int2: P2,
    int3: P3,
    int4: P4,
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin> Motor<P1, P2, P3, P4> {
    pub fn new(
        mut pin1: P1,
        mut pin2: P2,
        mut pin3: P3,
        mut pin4: P4
    ) -> Self {

        let mut motor = Motor {
            step: 0,
            int1: pin1,
            int2: pin2,
            int3: pin3,
            int4: pin4,
        };
        motor.stop();
        motor
    }

    fn step_forward(&mut self
    ) {
        self.do_step();
        if self.step == 3 {
            self.step = 0;
        } else {
            self.step += 1;
        }
    }

    fn step_back(&mut self
    ) {
        self.do_step();
        if self.step == 0 {
            self.step = 3;
        } else {
            self.step -= 1;
        }
    }

    fn stop(&mut self) {
        let _ = self.int1.set_low();
        let _ = self.int2.set_low();
        let _ = self.int3.set_low();
        let _ = self.int4.set_low();
    }

    fn do_step(&mut self) {
        match self.step {
            0 => {   // 1010
                self.int1.set_high();
                self.int2.set_low();
                self.int3.set_high();
                self.int4.set_low();
            }
            1 => {  // 0110
                self.int1.set_low();
                self.int2.set_high();
                self.int3.set_high();
                self.int4.set_low();
            }
            2 => {  // 0101
                self.int1.set_low();
                self.int2.set_high();
                self.int3.set_low();
                self.int4.set_high();
            }
            3 => {  // 1001
                self.int1.set_high();
                self.int2.set_low();
                self.int3.set_low();
                self.int4.set_high();
            }
            _ => {}
        }
    }

}
