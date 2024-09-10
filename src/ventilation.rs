use crate::relay_ctrl;
use crate::shared_data::AccessSharedData;
use crate::{config::Settings, relay_ctrl::RelayStatus};
use tokio::time::{sleep, Duration};

async fn trigger_ventilator(
    sd: &AccessSharedData,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("trigger_ventilator() -> turning ON ventilator for 60 seconds !");
    relay_ctrl::change_relay_status(settings.relay_pins.ventilator_or_heater, RelayStatus::On)
        .await?;
    (*sd).set_ventilator_status(RelayStatus::On);
    sleep(Duration::from_secs(settings.ventilation.duration)).await;
    println!("trigger_ventilator() -> turning OFF ventilator !");
    relay_ctrl::change_relay_status(settings.relay_pins.ventilator_or_heater, RelayStatus::Off)
        .await?;
    (*sd).set_ventilator_status(RelayStatus::Off);
    Ok(())
}

pub async fn ventilation_loop(sd: &AccessSharedData, settings: &Settings) {
    loop {
        if sd.ventilator_status() == RelayStatus::Off {
            if let Err(e) = trigger_ventilator(&sd, &settings).await {
                eprintln!("Error in trigger_ventilator: {:?}", e);
            }
        }
        sleep(Duration::from_secs(settings.ventilation.interval)).await;
    }
}
