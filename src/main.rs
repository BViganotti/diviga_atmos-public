pub mod config;
pub mod error;
pub mod initialization;
pub mod mock_relay_ctrl;
pub mod monitor_atmosphere;
pub mod read_atmosphere;
pub mod relay_ctrl;
pub mod request_atmosphere;
pub mod routes;
pub mod shared_data;
pub mod webserver;
use crate::config::Settings;
use crate::shared_data::AccessSharedData;
use dotenv::dotenv;
use relay_ctrl::RelayStatus;
use std::sync::Arc;
use std::sync::Mutex;
mod influx_client;
use crate::initialization::{initialize_relay_pins, initialize_shared_data};
use crate::request_atmosphere::request_atmosphere;
use influx_client::InfluxClient;

use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting atmospheric control system");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Initialize shared data and relay pins
    let common_data = initialize_shared_data();
    initialize_relay_pins(&settings).await?;

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
    let server = webserver::run_app(webserver_data)?;

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    atmosphere_handle.await?;

    Ok(())
}
