// Controls a relay connected to the Pi - by setting a pin high or low
// on demand.

use crate::error::AtmosError;
use log::{error, info, warn};
use rppal::gpio::Gpio;

pub const RELAY_IN1_PIN_HUMIDIFIER: u8 = 14;
pub const RELAY_IN2_PIN_DEHUMIDIFIER: u8 = 15;
pub const RELAY_IN3_PIN_VENTILATOR_OR_HEATER: u8 = 18;
pub const RELAY_IN4_PIN_FRIDGE: u8 = 17;

pub fn change_relay_status(pin_number: u8, relay_status: bool) -> Result<(), AtmosError> {
    let mut pin = Gpio::new()?
        .get(pin_number)
        .map_err(|_| AtmosError::RelayControlError(format!("Can't get pin {}", pin_number)))?
        .into_output();

    pin.set_reset_on_drop(false);

    match relay_status {
        false => {
            println!("setting pin high -> turning relay OFF");
            info!("Setting pin {} high -> turning relay OFF", pin_number);
            pin.set_high();
        }
        true => {
            println!("setting pin low -> turning relay ON");
            info!("Setting pin {} low -> turning relay ON", pin_number);
            pin.set_low();
        }
    }

    Ok(())
}
