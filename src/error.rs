use std::fmt;

#[derive(Debug)]
pub enum AtmosError {
    ConfigError(config::ConfigError),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    GpioError(rppal::gpio::Error),
    SensorReadError(String),
    RelayControlError(String),
    SqliteError(rusqlite::Error),
    JsonParseError(String),
    HttpError(String),
    FridgeError(String),
    DehumidifierError(String),
    HumidifierError(String),
    VentilatorError(String),
    RelayError(String),
    SensorError(String),
    TaskJoinError(String),
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
            AtmosError::SqliteError(e) => write!(f, "Sqlite error: {}", e),
            AtmosError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
            AtmosError::HttpError(e) => write!(f, "HTTP error: {}", e),
            AtmosError::FridgeError(e) => write!(f, "Fridge error: {}", e),
            AtmosError::DehumidifierError(e) => write!(f, "Dehumidifier error: {}", e),
            AtmosError::HumidifierError(e) => write!(f, "Humidifier error: {}", e),
            AtmosError::VentilatorError(e) => write!(f, "Ventilator error: {}", e),
            AtmosError::RelayError(e) => write!(f, "Relay error: {}", e),
            AtmosError::SensorError(e) => write!(f, "Sensor error: {}", e),
            AtmosError::TaskJoinError(e) => write!(f, "Task join error: {}", e),
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

impl From<rusqlite::Error> for AtmosError {
    fn from(err: rusqlite::Error) -> Self {
        AtmosError::SqliteError(err)
    }
}

impl From<tokio::task::JoinError> for AtmosError {
    fn from(err: tokio::task::JoinError) -> Self {
        AtmosError::TaskJoinError(err.to_string())
    }
}
