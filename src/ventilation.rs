use crate::relay_ctrl;
use crate::relay_ctrl::RELAY_IN3_PIN_VENTILATOR_OR_HEATER;
use std::thread;
use std::time::Duration;

const HALF_HOURS: u64 = 3600 / 2;

fn trigger_ventilator() {
    println!("trigger_ventilator() -> turning ON ventilator for 60 seconds !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, true)
        .expect("unable to change relay");
    thread::sleep(Duration::from_secs(60)); // a good minute for ventilation for the air to circulate
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false)
        .expect("unable to change relay");
}

pub fn ventilation_loop() {
    loop {
        trigger_ventilator();
        thread::sleep(Duration::from_secs(HALF_HOURS));
    }
}
