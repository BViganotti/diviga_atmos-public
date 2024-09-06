use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub temperature: TemperatureSettings,
    pub humidity: HumiditySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TemperatureSettings {
    pub low_range_min: f32,
    pub low_range_max: f32,
    pub high_range_min: f32,
    pub high_range_max: f32,
    pub ideal_range_min: f32,
    pub ideal_range_max: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HumiditySettings {
    pub low_range_min: f32,
    pub low_range_max: f32,
    pub high_range_min: f32,
    pub high_range_max: f32,
    pub ideal_range_min: f32,
    pub ideal_range_max: f32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .build()?;

        s.try_deserialize()
    }
}
