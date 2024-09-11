use crate::read_atmosphere;
use crate::shared_data::AccessSharedData;
use crate::{config::Settings, influx_client::InfluxClient};
use tokio::time::{sleep, Duration};

pub async fn request_atmosphere(
    sd: &AccessSharedData,
    settings: &Settings,
    influx_client: &InfluxClient,
) {
    loop {
        if let Err(e) = read_atmosphere::read_atmosphere_from_sensors(sd, influx_client).await {
            log::error!("Error reading atmosphere data: {:?}", e);
        }
        sleep(Duration::from_secs(settings.sensor_read_cooldown.duration)).await;
    }
}
