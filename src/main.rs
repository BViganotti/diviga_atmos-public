pub mod config;
//pub mod email_notification;
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
mod sqlite_client;
use crate::error::AtmosError;
use crate::initialization::{
    deinitialize_relay_pins, initialize_relay_pins, initialize_shared_data,
};
use crate::monitor_atmosphere::monitor_atmosphere;
use crate::request_atmosphere::request_atmosphere;
use sqlite_client::SqliteClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), AtmosError> {
    dotenv().ok();
    env_logger::init();
    log::info!("Starting atmospheric control system");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Initialize shared data and relay pins
    let common_data = initialize_shared_data();
    initialize_relay_pins(&settings).await?;

    // Create a channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);

    let shared_data = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    // Spawn the main task
    let main_task = tokio::spawn(run_main(
        shared_data.clone(),
        settings.clone(),
        shutdown_rx.resubscribe(),
    ));

    // Wait for Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
            // close all relays
            deinitialize_relay_pins(&settings, shutdown_rx.resubscribe()).await?;
        }
    }

    // Send shutdown signal
    shutdown_tx
        .send(())
        .expect("Failed to send shutdown signal");

    // Wait for the main task to finish
    main_task.await.expect("Main task panicked")?;

    println!("Shutdown complete");
    Ok(())
}

async fn run_main(
    shared_data: AccessSharedData,
    settings: Settings,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<(), AtmosError> {
    let sqlite_client = Arc::new(SqliteClient::new(&settings.sqlite.db_name)?);

    let monitor_shared_data = shared_data.clone();
    let monitor_settings = settings.clone();
    let monitor_sqlite_client = sqlite_client.clone();
    let monitor_task = tokio::spawn(monitor_atmosphere(
        monitor_shared_data,
        monitor_settings,
        monitor_sqlite_client,
        shutdown_rx.resubscribe(),
    ));

    let request_shared_data = shared_data.clone();
    let request_settings = settings.clone();
    let request_task = tokio::spawn(request_atmosphere(
        request_shared_data,
        request_settings,
        shutdown_rx.resubscribe(),
    ));

    let webserver_shared_data = shared_data.clone();
    let webserver_settings = settings.clone();
    let webserver_sqlite_client = sqlite_client.clone();
    let webserver_task = tokio::spawn(webserver::run_app(
        webserver_shared_data,
        webserver_settings,
        shutdown_rx.resubscribe(),
        webserver_sqlite_client,
    ));

    tokio::select! {
        _ = monitor_task => println!("Monitor task finished"),
        _ = request_task => println!("Request task finished"),
        _ = webserver_task => println!("Webserver task finished"),
        _ = shutdown_rx.recv() => println!("Received shutdown signal"),
    }

    Ok(())
}
