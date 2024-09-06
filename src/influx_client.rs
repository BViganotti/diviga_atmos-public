use chrono::{DateTime, Utc};
use influxdb::{Client, InfluxDbWriteable};
use serde::Serialize;

#[derive(InfluxDbWriteable, Serialize)]
pub struct AtmosphereData {
    time: DateTime<Utc>,
    #[influxdb(tag)]
    location: String,
    temperature: f32,
    humidity: f32,
}

#[derive(Clone)]
pub struct InfluxClient {
    client: Client,
}

impl InfluxClient {
    pub fn new(host: &str, db: &str) -> Self {
        let client = Client::new(host, db);
        InfluxClient { client }
    }

    pub async fn write_atmosphere_data(
        &self,
        location: &str,
        temperature: f32,
        humidity: f32,
    ) -> Result<(), influxdb::Error> {
        let data = AtmosphereData {
            time: Utc::now(),
            location: location.to_string(),
            temperature,
            humidity,
        };

        self.client
            .query(data.into_query("atmosphere_measurements"))
            .await?;
        Ok(())
    }
}
