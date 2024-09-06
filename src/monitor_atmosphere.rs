use crate::config::Settings;
use crate::relay_ctrl;
use crate::relay_ctrl::{RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE};
use crate::shared_data::AccessSharedData;
use log::{info, warn};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::time::sleep;

const FRIDGE_COOLDOWN_DURATION: Duration = Duration::from_secs(20 * 60); // 20 minutes

pub async fn atmosphere_monitoring(sd: AccessSharedData, settings: Settings) {
    info!("Starting atmosphere monitoring");
    loop {
        update_average_values(&sd);
        update_atmosphere_quality_index(&sd, &settings);

        if sd.polling_iterations() > 4 {
            control_environment(&sd, &settings);
        }

        info!(
            "Current atmosphere - Temp: {:.2}°C, Humidity: {:.2}%",
            sd.average_temp(),
            sd.average_humidity()
        );
        sleep(Duration::from_secs(60)).await;
    }
}

fn update_average_values(sd: &AccessSharedData) {
    sd.set_average_temp((sd.temp_one() + sd.temp_two()) / 2.0);
    sd.set_average_humidity((sd.humidity_one() + sd.humidity_two()) / 2.0);
}

fn update_atmosphere_quality_index(sd: &AccessSharedData, settings: &Settings) {
    let temp_in_range = settings.temperature.ideal_range_min <= sd.average_temp()
        && sd.average_temp() <= settings.temperature.ideal_range_max;
    let humidity_in_range = settings.humidity.ideal_range_min <= sd.average_humidity()
        && sd.average_humidity() <= settings.humidity.ideal_range_max;

    sd.set_atmosphere_quality_index(if temp_in_range && humidity_in_range {
        100.0
    } else {
        0.0
    });
}

fn control_environment(sd: &AccessSharedData, settings: &Settings) {
    let current_time = OffsetDateTime::now_utc();
    let time_since_last_change = current_time - sd.fridge_turn_off_datetime();

    if time_since_last_change >= FRIDGE_COOLDOWN_DURATION {
        if sd.average_temp() > settings.temperature.high_range_min {
            if !sd.fridge_status() {
                info!("Turning fridge ON");
                if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, true) {
                    warn!("Unable to change fridge relay: {:?}", e);
                } else {
                    sd.set_fridge_status(true);
                    sd.set_fridge_turn_on_datetime(current_time);
                }
            }
        } else if sd.average_temp() < settings.temperature.low_range_max {
            if sd.fridge_status() {
                info!("Turning fridge OFF");
                if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, false) {
                    warn!("Unable to change fridge relay: {:?}", e);
                } else {
                    sd.set_fridge_status(false);
                    sd.set_fridge_turn_off_datetime(current_time);
                }
            }
        }
    } else {
        info!("Fridge status change prevented due to cooldown period");
    }

    // Dehumidifier control (no cooldown period)
    if sd.average_humidity() > settings.humidity.high_range_min {
        if !sd.dehumidifier_status() {
            info!("Turning dehumidifier ON");
            if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, true) {
                warn!("Unable to change dehumidifier relay: {:?}", e);
            } else {
                sd.set_dehumidifier_status(true);
            }
        }
    } else if sd.average_humidity() < settings.humidity.low_range_max {
        if sd.dehumidifier_status() {
            info!("Turning dehumidifier OFF");
            if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false) {
                warn!("Unable to change dehumidifier relay: {:?}", e);
            } else {
                sd.set_dehumidifier_status(false);
            }
        }
    }
}
