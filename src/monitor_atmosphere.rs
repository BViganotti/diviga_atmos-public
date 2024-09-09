use crate::config::Settings;
use crate::error::AtmosError;
use crate::relay_ctrl;
use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE,
};
use crate::shared_data::AccessSharedData;
use log::{info, warn};
use std::time::Duration;
use time::{macros::offset, OffsetDateTime};
use tokio::time::sleep;

const FRIDGE_COOLDOWN_DURATION: Duration = Duration::from_secs(20 * 60); // 20 minutes
const HUMIDIFIER_COOLDOWN_DURATION: Duration = Duration::from_secs(10 * 60); // 10 minutes
const HUMIDIFIER_ACTIVATION_DURATION: Duration = Duration::from_secs(1); // 1 second

pub async fn atmosphere_monitoring(
    sd: AccessSharedData,
    settings: Settings,
) -> Result<(), AtmosError> {
    info!("Starting atmosphere monitoring");
    loop {
        update_average_values(&sd);
        update_atmosphere_quality_index(&sd, &settings);

        if sd.polling_iterations() > 4 {
            fridge_control(&sd, &settings).await?;
            dehumidifier_control(&sd, &settings).await?;
            humidifier_control(&sd, &settings).await?;
        }

        info!(
            "Current atmosphere - Temp: {:.2}°C, Humidity: {:.2}%",
            sd.average_temp(),
            sd.average_humidity()
        );
        sleep(Duration::from_secs(60)).await;
    }
}

async fn fridge_control(sd: &AccessSharedData, settings: &Settings) -> Result<(), AtmosError> {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    let temp_range = get_temperature_range(sd.average_temp(), settings);

    match temp_range {
        TempRange::High => handle_fridge_state(sd, now, true, "high temp range").await,
        TempRange::Ideal => handle_fridge_state(sd, now, false, "ideal temp range").await,
        TempRange::Low => handle_fridge_state(sd, now, false, "low temp range").await,
    }
}

async fn handle_fridge_state(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    target_state: bool,
    range_info: &str,
) -> Result<(), AtmosError> {
    info!("fridge_control() -> {}", range_info);

    if sd.fridge_status() != target_state {
        let time_since_last_change = if target_state {
            now - sd.fridge_turn_off_datetime()
        } else {
            now - sd.fridge_turn_on_datetime()
        };

        if time_since_last_change < FRIDGE_COOLDOWN_DURATION {
            info!(
                "fridge_control() -> waiting {:.2} minutes before turning {}",
                time_since_last_change.whole_minutes() as f64,
                if target_state { "on" } else { "off" }
            );
        } else {
            info!(
                "fridge_control() -> turning {} the fridge!",
                if target_state { "on" } else { "off" }
            );
            relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, target_state).await?;
            sd.set_fridge_status(target_state);
            if target_state {
                sd.set_fridge_turn_on_datetime(now);
            } else {
                sd.set_fridge_turn_off_datetime(now);
            }
        }
    }
    Ok(())
}

enum TempRange {
    High,
    Ideal,
    Low,
}

fn get_temperature_range(temp: f32, settings: &Settings) -> TempRange {
    if settings.temperature.high_range.contains(&temp) {
        TempRange::High
    } else if settings.temperature.ideal_range.contains(&temp) {
        TempRange::Ideal
    } else {
        TempRange::Low
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

async fn dehumidifier_control(
    sd: &AccessSharedData,
    settings: &Settings,
) -> Result<(), AtmosError> {
    let humidity_range = get_humidity_range(sd.average_humidity(), settings);

    match humidity_range {
        HumidityRange::High => handle_dehumidifier_state(sd, true, "high humidity range").await,
        HumidityRange::Ideal => handle_dehumidifier_state(sd, false, "ideal humidity range").await,
        HumidityRange::Low => handle_dehumidifier_state(sd, false, "low humidity range").await,
    }
}

async fn handle_dehumidifier_state(
    sd: &AccessSharedData,
    target_state: bool,
    range_info: &str,
) -> Result<(), AtmosError> {
    info!("dehumidifier_control() -> {}", range_info);

    if sd.dehumidifier_status() != target_state {
        info!(
            "dehumidifier_control() -> turning {} the dehumidifier!",
            if target_state { "on" } else { "off" }
        );
        relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, target_state).await?;
        sd.set_dehumidifier_status(target_state);
    }
    Ok(())
}

enum HumidityRange {
    High,
    Ideal,
    Low,
}

fn get_humidity_range(humidity: f32, settings: &Settings) -> HumidityRange {
    if settings.humidity.high_range.contains(&humidity) {
        HumidityRange::High
    } else if settings.humidity.ideal_range.contains(&humidity) {
        HumidityRange::Ideal
    } else {
        HumidityRange::Low
    }
}

async fn humidifier_control(sd: &AccessSharedData, settings: &Settings) -> Result<(), AtmosError> {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    let humidity_range = get_humidity_range(sd.average_humidity(), settings);
    let sd_clone = sd.clone();

    tokio::spawn(async move {
        match humidity_range {
            HumidityRange::Low => {
                handle_humidifier_state(&sd_clone, now, true, "low humidity range").await
            }
            _ => {
                handle_humidifier_state(&sd_clone, now, false, "ideal or high humidity range").await
            }
        }
    });

    Ok(())
}

async fn handle_humidifier_state(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    activate: bool,
    range_info: &str,
) {
    info!("humidifier_control() -> {}", range_info);

    let time_since_last_activation = now - sd.humidifier_turn_off_datetime();

    if activate && time_since_last_activation >= HUMIDIFIER_COOLDOWN_DURATION {
        info!("humidifier_control() -> activating humidifier for 1 second");
        match relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, true).await {
            Ok(_) => {
                tokio::time::sleep(HUMIDIFIER_ACTIVATION_DURATION).await;
                if let Err(e) =
                    relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false).await
                {
                    warn!("Unable to turn off humidifier relay: {:?}", e);
                    return;
                }
            }
            Err(e) => {
                warn!("Unable to turn on humidifier relay: {:?}", e);
                return;
            }
        }
    } else if activate {
        info!("humidifier_control() -> activation prevented due to cooldown period");
    }

    // The function doesn't need to return a Result since it's not propagating any errors
}
