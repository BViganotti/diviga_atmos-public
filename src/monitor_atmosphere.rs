use crate::config::Settings;
use crate::relay_ctrl;
use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE,
};
use crate::shared_data::AccessSharedData;
use log::{info, warn};
use std::time::Duration;
use time::OffsetDateTime;
use tokio::time::sleep;

const FRIDGE_COOLDOWN_DURATION: Duration = Duration::from_secs(20 * 60); // 20 minutes
const HUMIDIFIER_COOLDOWN_DURATION: Duration = Duration::from_secs(10 * 60); // 10 minutes
const HUMIDIFIER_ACTIVATION_DURATION: Duration = Duration::from_secs(1); // 1 second

pub async fn atmosphere_monitoring(sd: AccessSharedData, settings: Settings) {
    info!("Starting atmosphere monitoring");
    loop {
        update_average_values(&sd);
        update_atmosphere_quality_index(&sd, &settings);

        if sd.polling_iterations() > 4 {
            fridge_control(&sd, &settings);
            dehumidifier_control(&sd, &settings);
            humidifier_control(&sd, &settings);
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
    let temp_in_range = settings
        .temperature
        .ideal_range
        .contains(&sd.average_temp());
    let humidity_in_range = settings
        .humidity
        .ideal_range
        .contains(&sd.average_humidity());

    sd.set_atmosphere_quality_index(if temp_in_range && humidity_in_range {
        100.0
    } else {
        0.0
    });
}

fn fridge_control(sd: &AccessSharedData, settings: &Settings) {
    let current_time = OffsetDateTime::now_utc();
    let time_since_last_change = current_time - sd.fridge_turn_off_datetime();

    if time_since_last_change >= FRIDGE_COOLDOWN_DURATION {
        if settings.temperature.high_range.contains(&sd.average_temp()) {
            if !sd.fridge_status() {
                info!("Turning fridge ON");
                if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, true) {
                    warn!("Unable to change fridge relay: {:?}", e);
                } else {
                    sd.set_fridge_status(true);
                    sd.set_fridge_turn_on_datetime(current_time);
                }
            }
        } else if settings.temperature.low_range.contains(&sd.average_temp()) {
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
}

fn dehumidifier_control(sd: &AccessSharedData, settings: &Settings) {
    if settings
        .humidity
        .high_range
        .contains(&sd.average_humidity())
    {
        if !sd.dehumidifier_status() {
            info!("Turning dehumidifier ON");
            if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, true) {
                warn!("Unable to change dehumidifier relay: {:?}", e);
            } else {
                sd.set_dehumidifier_status(true);
            }
        }
    } else if settings.humidity.low_range.contains(&sd.average_humidity()) {
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

fn humidifier_control(sd: &AccessSharedData, settings: &Settings) {
    let current_time = OffsetDateTime::now_utc();
    let time_since_last_humidifier_activation = current_time - sd.humidifier_turn_off_datetime();
    if time_since_last_humidifier_activation >= HUMIDIFIER_COOLDOWN_DURATION {
        if settings.humidity.low_range.contains(&sd.average_humidity()) {
            info!("Activating humidifier for 1 second");
            if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, true) {
                warn!("Unable to turn on humidifier relay: {:?}", e);
            } else {
                std::thread::sleep(HUMIDIFIER_ACTIVATION_DURATION);
                if let Err(e) = relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false) {
                    warn!("Unable to turn off humidifier relay: {:?}", e);
                } else {
                    sd.set_humidifier_status(false);
                    sd.set_humidifier_turn_off_datetime(current_time);
                }
            }
        }
    } else {
        info!("Humidifier activation prevented due to cooldown period");
    }
}
