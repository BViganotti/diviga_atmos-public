use crate::config::Settings;
use crate::error::AtmosError;
use crate::read_atmosphere;
use crate::shared_data::AccessSharedData;
use log::{error, info};
use tokio::time::{sleep, Duration};

pub async fn request_atmosphere(
    sd: AccessSharedData,
    settings: Settings,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
) -> Result<(), AtmosError> {
    loop {
        match read_atmosphere::read_atmosphere_from_sensors(&sd).await {
            Ok(_) => {
                // Successfully read atmosphere data
            }
            Err(e) => {
                error!("Error reading atmosphere data: {:?}", e);
                // You might want to handle specific error types differently here
            }
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(settings.sensor_read_cooldown.duration)) => {
                // Continue to next iteration
            }
            _ = shutdown_rx.recv() => {
                info!("Received shutdown signal");
                break Ok(());
            }
        }
    }
}
