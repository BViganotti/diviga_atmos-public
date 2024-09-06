use crate::influx_client::InfluxClient;
use crate::read_atmosphere;
use crate::shared_data::AccessSharedData;
use tokio::time::{sleep, Duration};

pub async fn request_atmosphere(sd: &AccessSharedData, influx_client: &InfluxClient) {
    loop {
        if let Err(e) = read_atmosphere::read_atmosphere_from_sensors(sd, influx_client).await {
            log::error!("Error reading atmosphere data: {:?}", e);
        }
        sleep(Duration::from_secs(45)).await;
    }
}
