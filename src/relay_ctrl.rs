// Controls a relay connected to the Pi - by setting a pin high or low
// on demand.

use crate::error::AtmosError;
use log::info;
use rppal::gpio::Gpio;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum RelayStatus {
    On,
    Off,
}

pub async fn change_relay_status(
    pin_number: u8,
    relay_status: RelayStatus,
) -> Result<(), AtmosError> {
    let mut pin = Gpio::new()?
        .get(pin_number)
        .map_err(|_| AtmosError::RelayControlError(format!("Can't get pin {}", pin_number)))?
        .into_output();

    pin.set_reset_on_drop(false);

    match relay_status {
        RelayStatus::Off => {
            info!("Setting pin {} high -> turning relay OFF", pin_number);
            pin.set_high();
        }
        RelayStatus::On => {
            info!("Setting pin {} low -> turning relay ON", pin_number);
            pin.set_low();
        }
    }

    Ok(())
}
