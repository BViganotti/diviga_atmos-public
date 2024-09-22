use crate::config::Settings;
use crate::error::AtmosError;
use crate::read_atmosphere;
use crate::shared_data::AccessSharedData;
use log::error;
use tokio::time::{sleep, Duration};

pub async fn request_atmosphere(
    sd: &AccessSharedData,
    settings: &Settings,
) -> Result<(), AtmosError> {
    loop {
        match read_atmosphere::read_atmosphere_from_sensors(sd).await {
            Ok(_) => {
                // Successfully read atmosphere data
            }
            Err(e) => {
                error!("Error reading atmosphere data: {:?}", e);
                // You might want to handle specific error types differently here
            }
        }

        sleep(Duration::from_secs(settings.sensor_read_cooldown.duration)).await;
    }
}
