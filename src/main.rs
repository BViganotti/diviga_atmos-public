pub mod config;
pub mod error;
pub mod mock_relay_ctrl;
pub mod monitor_atmosphere;
pub mod read_atmosphere;
pub mod relay_ctrl;
pub mod request_atmosphere;
pub mod routes;
pub mod shared_data;
pub mod ventilation;
pub mod webserver;
use crate::config::Settings;
use crate::shared_data::AccessSharedData;
use crate::shared_data::SharedData;
use dotenv::dotenv;
use relay_ctrl::RelayStatus;
use std::sync::Arc;
use std::sync::Mutex;
use time::macros::offset;
use time::OffsetDateTime;
mod influx_client;
use crate::request_atmosphere::request_atmosphere;
use influx_client::InfluxClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting atmospheric control system");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Initialize a struct that will be our "global" data, which allows safe access from every thread
    let common_data = SharedData::new(
        0,
        13.0,
        80.0,
        13.0,
        80.0,
        0.0,
        80.0,
        0.0,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
    );

    // setting all the pins to false just in case
    for pin in &[
        settings.relay_pins.humidifier,
        settings.relay_pins.dehumidifier,
        settings.relay_pins.ventilator_or_heater,
        settings.relay_pins.fridge,
    ] {
        relay_ctrl::change_relay_status(*pin, RelayStatus::Off)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    }

    // The wrapper around our shared data that gives it safe access across threads
    let shared_data = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    let influx_client = InfluxClient::new(&settings.influxdb.host, &settings.influxdb.database);

    // Clone for atmosphere request task
    let atmosphere_data = shared_data.clone();
    let atmosphere_settings = settings.clone();
    let atmosphere_influx_client = influx_client.clone();

    let atmosphere_handle = tokio::spawn(async move {
        request_atmosphere(
            &atmosphere_data,
            &atmosphere_settings,
            &atmosphere_influx_client,
        )
        .await;
    });

    // Clone for ventilation task
    let ventilation_data = shared_data.clone();
    let ventilation_settings = settings.clone();
    tokio::spawn(async move {
        ventilation::ventilation_loop(&ventilation_data, &ventilation_settings).await;
    });

    // Clone for atmosphere monitoring task
    let monitoring_data = shared_data.clone();
    let monitoring_settings = settings.clone();
    tokio::spawn(async move {
        if let Err(e) =
            monitor_atmosphere::monitor_atmosphere(monitoring_data, monitoring_settings).await
        {
            log::error!("Atmosphere monitoring error: {}", e);
        }
    });

    // Clone for webserver task
    let webserver_data = shared_data.clone();
    let webserver_settings = settings.clone();
    tokio::spawn(async move {
        let server_future = webserver::run_app(&webserver_data, &webserver_settings);
        server_future.await
    });

    atmosphere_handle.await?;

    Ok(())
}
