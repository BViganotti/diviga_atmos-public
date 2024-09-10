use crate::error::AtmosError;

#[cfg(not(test))]
use rppal::gpio::Gpio;

#[cfg(test)]
use crate::mock_relay_ctrl::{get_mock_relay_status, mock_change_relay_status};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RelayStatus {
    On,
    Off,
}

#[cfg(not(test))]
pub async fn change_relay_status(pin: u8, status: RelayStatus) -> Result<(), AtmosError> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(pin)?.into_output();
    match status {
        RelayStatus::On => pin.set_high(),
        RelayStatus::Off => pin.set_low(),
    }
    Ok(())
}

#[cfg(test)]
pub async fn change_relay_status(pin: u8, status: RelayStatus) -> Result<(), AtmosError> {
    mock_change_relay_status(pin, status).await
}

#[cfg(test)]
pub fn get_relay_status(pin: u8) -> RelayStatus {
    get_mock_relay_status(pin)
}
