use crate::relay_ctrl;
use crate::relay_ctrl::RELAY_IN3_PIN_VENTILATOR_OR_HEATER;
use crate::shared_data::AccessSharedData;
use tokio::time::{sleep, Duration};

const VENTILATION_INTERVAL: Duration = Duration::from_secs(1800); // 30 minutes in seconds

async fn trigger_ventilator(sd: &AccessSharedData) -> Result<(), Box<dyn std::error::Error>> {
    println!("trigger_ventilator() -> turning ON ventilator for 60 seconds !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, true).await?;
    (*sd).set_ventilator_status(true);
    sleep(Duration::from_secs(60)).await;
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false).await?;
    (*sd).set_ventilator_status(false);
    Ok(())
}

pub async fn ventilation_loop(sd: AccessSharedData) {
    loop {
        if sd.ventilator_status() {
            if let Err(e) = trigger_ventilator(&sd).await {
                eprintln!("Error in trigger_ventilator: {:?}", e);
            }
        }
        sleep(VENTILATION_INTERVAL).await;
    }
}
