use crate::relay_ctrl;
use crate::relay_ctrl::RELAY_IN3_PIN_VENTILATOR_OR_HEATER;
use crate::shared_data::AccessSharedData;
use tokio::time::{sleep, Duration};

const VENTILATION_INTERVAL: Duration = Duration::from_secs(1800); // 30 minutes in seconds

async fn trigger_ventilator(sd: &AccessSharedData) {
    println!("trigger_ventilator() -> turning ON ventilator for 60 seconds !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, true)
        .expect("unable to change relay");
    sd.set_ventilator_status(true);
    sleep(Duration::from_secs(60)).await; // a good minute for ventilation for the air to circulate
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false)
        .expect("unable to change relay");
    sd.set_ventilator_status(false);
}

pub async fn ventilation_loop(sd: AccessSharedData) {
    loop {
        if sd.ventilator_status() {
            trigger_ventilator(&sd).await;
        }
        sleep(VENTILATION_INTERVAL).await;
    }
}
