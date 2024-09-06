// Controls a relay connected to the Pi - by setting a pin high or low
// on demand.

use rppal::gpio::Gpio;
use std::error::Error;

pub const RELAY_IN1_PIN_HUMIDIFIER: u8 = 14;
pub const RELAY_IN2_PIN_DEHUMIDIFIER: u8 = 15;
pub const RELAY_IN3_PIN_VENTILATOR_OR_HEATER: u8 = 18;
pub const RELAY_IN4_PIN_FRIDGE: u8 = 17;

pub fn change_relay_status(pin_number: u8, relay_status: bool) -> Result<(), Box<dyn Error>> {
    // Grab a handle to the pin we want to control, and set it up to be an
    // output pin
    let mut pin = Gpio::new()
        .expect("gpio failed")
        .get(pin_number)
        .expect("can't get pin")
        .into_output();

    // Set this so the pin stays put after leaving this function - and having
    // pin fall out of scope. Default behavior is to reset the pin back to low (false)
    // when pin falls out of scope. This is simpler than trying to keep pin as a long-running
    // (static) value.
    pin.set_reset_on_drop(false);

    // set the pin according to the relay_status
    match relay_status {
        false => {
            println!("setting pin high -> turning relay OFF");
            pin.set_high();
        }
        true => {
            println!("setting pin low -> turning relay ON");
            pin.set_low();
        }
    }

    Ok(())
}
