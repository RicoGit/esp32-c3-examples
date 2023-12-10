//! Post to MQTT topic and read value back.
//!
//! `cargo run --example mqtt`

use embedded_svc::mqtt::client::{Event, QoS};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // WIFI

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;
    connect_wifi(&mut wifi)?;

    // MQTT
    let conf = MqttClientConfiguration {
        username: Some(CONFIG.mqtt_user),
        password: Some(CONFIG.mqtt_password),
        ..Default::default()
    };
    let mut mqtt = EspMqttClient::new(CONFIG.mqtt_url, &conf, |res| {
        log::info!("Mqtt > {:?}", res);
        match res {
            Ok(Event::Received(msg)) => {
                log::info!("> recieved msg: {:?}", String::from_utf8_lossy(msg.data()))
            }
            _ => {}
        }
    })
    .unwrap();

    let topic = "esp32c3/rust/example";
    mqtt.subscribe(topic, QoS::AtLeastOnce).unwrap();

    loop {
        FreeRtos::delay_ms(5000);
        mqtt.publish(
            topic,
            QoS::AtLeastOnce,
            false,
            "Hello from Rust and Esp32-C3!".as_bytes(),
        )
        .unwrap();
    }
}

#[derive(Debug)]
#[toml_cfg::toml_config]
struct Config {
    #[default("NO SSID")]
    wifi_ssid: &'static str,
    #[default("NO PASSWORD")]
    wifi_password: &'static str,

    #[default(1883)]
    mqtt_port: u16,
    #[default("NO MQTT URL")]
    mqtt_url: &'static str,
    #[default("NO MQTT USER")]
    mqtt_user: &'static str,
    #[default("NO MQTT PASSWORD")]
    mqtt_password: &'static str,
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> eyre::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: CONFIG.wifi_ssid.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: CONFIG.wifi_password.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    Ok(())
}
