use crate::error::AtmosError;
use rppal::gpio::Gpio;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelayStatus {
    On,
    Off,
}

impl fmt::Display for RelayStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RelayStatus::On => write!(f, "On"),
            RelayStatus::Off => write!(f, "Off"),
        }
    }
}

pub async fn change_relay_status(pin: u8, status: RelayStatus) -> Result<(), AtmosError> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(pin)?.into_output();
    match status {
        RelayStatus::On => pin.set_high(),
        RelayStatus::Off => pin.set_low(),
    }
    Ok(())
}
