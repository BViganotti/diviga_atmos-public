use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::fmt;

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

        let settings: Settings = s.try_deserialize()?;
        settings.validate()?;
        Ok(settings)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        self.temperature.validate()?;
        self.humidity.validate()?;
        Ok(())
    }
}

impl TemperatureSettings {
    fn validate(&self) -> Result<(), ConfigError> {
        if !(self.low_range_min <= self.low_range_max
            && self.low_range_max < self.ideal_range_min
            && self.ideal_range_min <= self.ideal_range_max
            && self.ideal_range_max < self.high_range_min
            && self.high_range_min <= self.high_range_max)
        {
            return Err(ConfigError::Message("Invalid temperature ranges".into()));
        }
        Ok(())
    }
}

impl HumiditySettings {
    fn validate(&self) -> Result<(), ConfigError> {
        if !(self.low_range_min <= self.low_range_max
            && self.low_range_max < self.ideal_range_min
            && self.ideal_range_min <= self.ideal_range_max
            && self.ideal_range_max < self.high_range_min
            && self.high_range_min <= self.high_range_max)
        {
            return Err(ConfigError::Message("Invalid humidity ranges".into()));
        }
        Ok(())
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Temperature: {:?}\nHumidity: {:?}",
            self.temperature, self.humidity
        )
    }
}
