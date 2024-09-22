use crate::error::AtmosError;
use log::{debug, error, info};
use rppal::gpio::{Gpio, Level};
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

pub fn change_relay_status(pin: u8, status: RelayStatus) -> Result<(), AtmosError> {
    info!(
        "Attempting to change relay status for pin {} to {:?}",
        pin, status
    );
    let gpio = Gpio::new()?;
    let mut pin_obj = gpio.get(pin)?.into_output();

    // Set this so the pin stays put after leaving this function - and having
    // pin fall out of scope. Default behavior is to reset the pin back to low (false)
    // when pin falls out of scope. This is simpler than trying to keep pin as a long-running
    // (static) value.
    pin_obj.set_reset_on_drop(false);

    match status {
        RelayStatus::On => {
            pin_obj.set_low(); // Set low to turn on (active-low)
            debug!("Set pin {} to low (RelayStatus::On)", pin);
        }
        RelayStatus::Off => {
            pin_obj.set_high(); // Set high to turn off (active-low)
            debug!("Set pin {} to high (RelayStatus::Off)", pin);
        }
    }
    info!(
        "Successfully changed relay status for pin {} to {:?}",
        pin, status
    );
    Ok(())
}

pub fn check_relay_status(pin: u8) -> Result<RelayStatus, AtmosError> {
    info!("Checking relay status for pin {}", pin);
    let gpio = Gpio::new()?;
    let pin_obj = gpio.get(pin)?.into_input();
    let status = match pin_obj.read() {
        Level::Low => {
            debug!("Pin {} is Low, interpreting as RelayStatus::On", pin);
            Ok(RelayStatus::On)
        }
        Level::High => {
            debug!("Pin {} is High, interpreting as RelayStatus::Off", pin);
            Ok(RelayStatus::Off)
        }
    };
    info!("Relay status for pin {} is {:?}", pin, status);
    status
}
