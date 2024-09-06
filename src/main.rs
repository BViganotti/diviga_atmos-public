/*
This project is very very much influenced and heavily copy pasted from https://github.com/mikehentges/thermostat-pi.
*/

pub mod config;
pub mod error;
pub mod monitor_atmosphere;
pub mod read_atmosphere;
pub mod relay_ctrl;
pub mod request_atmosphere;
pub mod routes;
pub mod shared_data;
pub mod ventilation;
pub mod webserver;

use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN3_PIN_VENTILATOR_OR_HEATER,
    RELAY_IN4_PIN_FRIDGE,
};
use crate::shared_data::AccessSharedData;
use crate::shared_data::SharedData;
use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use time::macros::offset;
use time::OffsetDateTime;

use crate::config::Settings;

mod influx_client;
use influx_client::InfluxClient;

use crate::request_atmosphere::request_atmosphere;

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
        false,
        false,
        false,
        false,
        false, // Add this line for heater_status
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
    relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, false)
        .expect("unable to initialize relay");

    // The wrapper around our shared data that gives it safe access across threads
    let shared_data = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    let influx_client = InfluxClient::new("http://localhost:8086", "atmosphere_db");

    // Clone for atmosphere request task
    let atmosphere_data = shared_data.clone();
    let atmosphere_influx_client = influx_client.clone();

    let atmosphere_handle = tokio::spawn(async move {
        request_atmosphere(&atmosphere_data, &atmosphere_influx_client).await;
    });

    // Clone for ventilation task
    let ventilation_data = shared_data.clone();
    tokio::spawn(async move {
        ventilation::ventilation_loop(ventilation_data).await;
    });

    // Clone for atmosphere monitoring task
    let monitoring_data = shared_data.clone();
    let monitoring_settings = settings.clone();
    tokio::spawn(async move {
        monitor_atmosphere::atmosphere_monitoring(monitoring_data, monitoring_settings).await;
    });

    // Clone for webserver task
    let webserver_data = shared_data.clone();
    tokio::spawn(async move {
        let server_future = webserver::run_app(&webserver_data);
        server_future.await
    });

    atmosphere_handle.await?;

    Ok(())
}
