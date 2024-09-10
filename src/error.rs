use influxdb::Error as InfluxDbError;
use std::fmt;

#[derive(Debug)]
pub enum AtmosError {
    ConfigError(config::ConfigError),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    GpioError(rppal::gpio::Error),
    SensorReadError(String),
    RelayControlError(String),
    InfluxDbError(InfluxDbError),
    JsonParseError(String),
    HttpError(String),
}

impl fmt::Display for AtmosError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AtmosError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            AtmosError::IoError(e) => write!(f, "I/O error: {}", e),
            AtmosError::JsonError(e) => write!(f, "JSON error: {}", e),
            AtmosError::GpioError(e) => write!(f, "GPIO error: {}", e),
            AtmosError::SensorReadError(e) => write!(f, "Sensor read error: {}", e),
            AtmosError::RelayControlError(e) => write!(f, "Relay control error: {}", e),
            AtmosError::InfluxDbError(e) => write!(f, "InfluxDB error: {}", e),
            AtmosError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            AtmosError::HttpError(e) => write!(f, "HTTP error: {}", e),
        }
    }
}

impl std::error::Error for AtmosError {}

impl From<config::ConfigError> for AtmosError {
    fn from(err: config::ConfigError) -> Self {
        AtmosError::ConfigError(err)
    }
}

impl From<std::io::Error> for AtmosError {
    fn from(err: std::io::Error) -> Self {
        AtmosError::IoError(err)
    }
}

impl From<serde_json::Error> for AtmosError {
    fn from(error: serde_json::Error) -> Self {
        AtmosError::JsonParseError(error.to_string())
    }
}

impl From<rppal::gpio::Error> for AtmosError {
    fn from(err: rppal::gpio::Error) -> Self {
        AtmosError::GpioError(err)
    }
}

impl From<InfluxDbError> for AtmosError {
    fn from(error: InfluxDbError) -> Self {
        AtmosError::InfluxDbError(error)
    }
}
