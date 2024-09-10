use crate::error::AtmosError;
use config::{Config, File};
use serde::Deserialize;
use std::fmt;
use std::ops::Range;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub temperature: TemperatureSettings,
    pub humidity: HumiditySettings,
    pub ventilation: VentilationSettings,
    pub influxdb: InfluxDbSettings,
    pub relay_pins: RelayPinSettings,
    pub webserver: WebserverSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TemperatureSettings {
    pub low_range: Range<f32>,
    pub high_range: Range<f32>,
    pub ideal_range: Range<f32>,
    pub fridge_cooldown_duration: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HumiditySettings {
    pub low_range: Range<f32>,
    pub high_range: Range<f32>,
    pub ideal_range: Range<f32>,
    pub humidifier_cooldown_duration: u64,
    pub humidifier_activation_duration: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VentilationSettings {
    pub interval: u64,
    pub duration: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InfluxDbSettings {
    pub host: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RelayPinSettings {
    pub humidifier: u8,
    pub dehumidifier: u8,
    pub ventilator_or_heater: u8,
    pub fridge: u8,
}

impl Settings {
    pub fn new() -> Result<Self, AtmosError> {
        let s = Config::builder()
            .add_source(File::with_name("config"))
            .add_source(config::Environment::with_prefix("ATMOS"))
            .build()?;

        let settings: Settings = s.try_deserialize()?;
        settings.validate()?;
        Ok(settings)
    }

    fn validate(&self) -> Result<(), AtmosError> {
        self.temperature.validate()?;
        self.humidity.validate()?;
        Ok(())
    }
}

impl TemperatureSettings {
    fn validate(&self) -> Result<(), AtmosError> {
        if !(self.low_range.start <= self.low_range.end
            && self.low_range.end < self.ideal_range.start
            && self.ideal_range.start <= self.ideal_range.end
            && self.ideal_range.end < self.high_range.start
            && self.high_range.start <= self.high_range.end)
        {
            return Err(AtmosError::ConfigError(config::ConfigError::Message(
                "Invalid temperature ranges".into(),
            )));
        }
        Ok(())
    }
}

impl HumiditySettings {
    fn validate(&self) -> Result<(), AtmosError> {
        if !(self.low_range.start <= self.low_range.end
            && self.low_range.end < self.ideal_range.start
            && self.ideal_range.start <= self.ideal_range.end
            && self.ideal_range.end < self.high_range.start
            && self.high_range.start <= self.high_range.end)
        {
            return Err(AtmosError::ConfigError(config::ConfigError::Message(
                "Invalid humidity ranges".into(),
            )));
        }
        Ok(())
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Temperature: {:?}\nHumidity: {:?}\nVentilation: {:?}\nInfluxDB: {:?}",
            self.temperature, self.humidity, self.ventilation, self.influxdb
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebserverSettings {
    pub host: String,
    pub port: u16,
}

impl fmt::Display for WebserverSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Webserver: {:?}", self)
    }
}
