//! Connectes to MQTT (via WiFi), publishes a message and reads it back.
//!
//! `cargo run --example mqtt`

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use esp_idf_svc::http::{client as http_client, Method};

fn main() -> eyre::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    dbg!(CONFIG);

    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Wifi DHCP info: {:?}", ip_info);

    let http_config = http_client::Configuration::default();
    let mut client = http_client::EspHttpConnection::new(&http_config).expect("http client should be created");

    client.initiate_request(Method::Get, "http://example.com", &[]).expect("http GET request should be ok");
    log::info!("response status: {:?}", client.status());
    let mut buf = [0u8; 1024];
    client.read(&mut buf).expect("http read should be ok");
    log::info!("response body: {:?}", buf);

    Ok(())
}

#[derive(Debug)]
#[toml_cfg::toml_config]
struct Config {
    #[default("NO SSID")]
    wifi_ssid: &'static str,
    #[default("NO PASSWORD")]
    wifi_password: &'static str,
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

