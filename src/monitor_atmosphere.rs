use crate::{
    error::AtmosError, relay_ctrl::change_relay_status, AccessSharedData, RelayStatus, Settings,
};
use futures::future::join_all;
use log::{debug, error, info};
use time::OffsetDateTime;
use tokio::time::{interval, Duration};

pub async fn monitor_atmosphere(
    sd: AccessSharedData,
    settings: Settings,
) -> Result<(), AtmosError> {
    let mut interval = interval(Duration::from_secs(settings.polling_interval.duration));

    loop {
        interval.tick().await;

        update_average_values(&sd);
        update_atmosphere_quality_index(&sd, &settings);

        if sd.polling_iterations() > 4 {
            let now = OffsetDateTime::now_utc();
            let tasks = vec![
                tokio::spawn(handle_fridge(sd.clone(), now, settings.clone())),
                tokio::spawn(handle_dehumidifier(sd.clone(), now, settings.clone())),
                tokio::spawn(handle_humidifier(sd.clone(), now, settings.clone())),
            ];

            for result in join_all(tasks).await {
                if let Err(e) = result {
                    error!("Task error: {:?}", e);
                }
            }
        } else {
            debug!(
                "Not enough polling iterations yet: {}",
                sd.polling_iterations()
            );
        }

        log_atmosphere_data(&sd);
    }
}

async fn handle_fridge(
    sd: AccessSharedData,
    now: OffsetDateTime,
    settings: Settings,
) -> Result<(), AtmosError> {
    let average_temp = sd.average_temp();
    let target_state = if settings.temperature.high_range().contains(&average_temp) {
        RelayStatus::On
    } else if settings.temperature.ideal_range().contains(&average_temp)
        || settings.temperature.low_range().contains(&average_temp)
    {
        RelayStatus::Off
    } else {
        return Ok(());
    };

    handle_relay_state(
        &sd,
        now,
        target_state,
        "fridge",
        &settings,
        settings.relay_pins.fridge,
        sd.fridge_status(),
        sd.fridge_turn_on_datetime(),
        sd.fridge_turn_off_datetime(),
        |sd, status| sd.set_fridge_status(status),
        settings.temperature.fridge_cooldown_duration,
    )
    .await
}

async fn handle_dehumidifier(
    sd: AccessSharedData,
    now: OffsetDateTime,
    settings: Settings,
) -> Result<(), AtmosError> {
    let average_humidity = sd.average_humidity();
    let target_state = if settings.humidity.high_range().contains(&average_humidity) {
        RelayStatus::On
    } else {
        RelayStatus::Off
    };

    handle_relay_state(
        &sd,
        now,
        target_state,
        "dehumidifier",
        &settings,
        settings.relay_pins.dehumidifier,
        sd.dehumidifier_status(),
        sd.dehumidifier_turn_on_datetime(),
        sd.dehumidifier_turn_off_datetime(),
        |sd, status| sd.set_dehumidifier_status(status),
        0, // Assuming no cooldown for dehumidifier
    )
    .await
}

async fn handle_humidifier(
    sd: AccessSharedData,
    now: OffsetDateTime,
    settings: Settings,
) -> Result<(), AtmosError> {
    let average_humidity = sd.average_humidity();
    if settings.humidity.low_range().contains(&average_humidity) {
        let time_since_last_activation = now - sd.humidifier_turn_off_datetime();
        if time_since_last_activation
            >= Duration::from_secs(settings.humidity.humidifier_cooldown_duration)
        {
            info!(
                "humidifier_control() -> activating humidifier for {} second(s)",
                settings.humidity.humidifier_activation_duration
            );
            change_relay_status(settings.relay_pins.humidifier, RelayStatus::On).await?;
            tokio::time::sleep(Duration::from_secs(
                settings.humidity.humidifier_activation_duration,
            ))
            .await;
            change_relay_status(settings.relay_pins.humidifier, RelayStatus::Off).await?;
            sd.set_humidifier_turn_off_datetime(now);
        } else {
            info!("humidifier_control() -> activation prevented due to cooldown period");
        }
    }
    Ok(())
}

async fn handle_relay_state(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    target_state: RelayStatus,
    device: &str,
    _settings: &Settings,
    pin_number: u8,
    current_status: RelayStatus,
    turn_on_datetime: OffsetDateTime,
    turn_off_datetime: OffsetDateTime,
    set_status: impl Fn(&AccessSharedData, RelayStatus),
    cooldown_duration: u64,
) -> Result<(), AtmosError> {
    info!(
        "{}_control() -> current state: {:?}, target state: {:?}",
        device, current_status, target_state
    );

    if current_status != target_state {
        let time_since_last_change = if target_state == RelayStatus::On {
            now - turn_off_datetime
        } else {
            now - turn_on_datetime
        };

        if time_since_last_change.whole_seconds() as u64 > cooldown_duration {
            info!(
                "{}_control() -> changing state to {:?}",
                device, target_state
            );
            change_relay_status(pin_number, target_state).await?;
            set_status(sd, target_state);
        } else {
            info!(
                "{}_control() -> waiting {:.2} minutes before changing state",
                device,
                time_since_last_change.whole_minutes() as f64
            );
        }
    }

    Ok(())
}

fn update_average_values(sd: &AccessSharedData) {
    sd.set_average_temp((sd.temp_one() + sd.temp_two()) / 2.0);
    sd.set_average_humidity((sd.humidity_one() + sd.humidity_two()) / 2.0);
}

fn update_atmosphere_quality_index(sd: &AccessSharedData, settings: &Settings) {
    let temp_in_range = settings
        .temperature
        .ideal_range()
        .contains(&sd.average_temp());
    let humidity_in_range = settings
        .humidity
        .ideal_range()
        .contains(&sd.average_humidity());

    sd.set_atmosphere_quality_index(if temp_in_range && humidity_in_range {
        100.0
    } else {
        0.0
    });
}

fn log_atmosphere_data(sd: &AccessSharedData) {
    info!(
        "Current atmosphere - Temp: {:.2}°C, Humidity: {:.2}%, Quality Index: {:.2}",
        sd.average_temp(),
        sd.average_humidity(),
        sd.atmosphere_quality_index()
    );
    info!(
        "Detailed readings - Temp1: {:.2}°C, Humidity1: {:.2}%, Temp2: {:.2}°C, Humidity2: {:.2}%",
        sd.temp_one(),
        sd.humidity_one(),
        sd.temp_two(),
        sd.humidity_two()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_relay_ctrl::get_mock_relay_status;
    use crate::{config::Settings, shared_data::SharedData};
    use std::sync::{Arc, Mutex};
    use time::macros::offset;

    fn create_test_shared_data() -> AccessSharedData {
        let test_data = SharedData::new(
            0,
            13.0,
            80.0,
            13.0,
            80.0,
            0.0,
            80.0,
            0.0,
            RelayStatus::Off,
            RelayStatus::Off,
            RelayStatus::Off,
            RelayStatus::Off,
            RelayStatus::Off,
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
            OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        );

        let shared_data = AccessSharedData {
            sd: Arc::new(Mutex::new(test_data)),
        };
        shared_data
    }

    #[tokio::test]
    async fn test_handle_fridge() {
        let sd = create_test_shared_data();
        let mut settings = Settings::new().unwrap();
        settings.temperature.high_range().start = 25.0;
        settings.temperature.high_range().end = 30.0;
        settings.temperature.ideal_range().start = 20.0;
        settings.temperature.ideal_range().end = 25.0;
        settings.temperature.low_range().start = 15.0;
        settings.temperature.low_range().end = 20.0;
        settings.relay_pins.fridge = 1;

        // Test when temperature is in high range
        sd.set_average_temp(26.0);
        handle_fridge(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.fridge),
            RelayStatus::On
        );

        // Test when temperature is in ideal range
        sd.set_average_temp(22.0);
        handle_fridge(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.fridge),
            RelayStatus::Off
        );
    }

    #[tokio::test]
    async fn test_handle_dehumidifier() {
        let sd = create_test_shared_data();
        let mut settings = Settings::new().unwrap();
        settings.humidity.high_range().start = 60.0;
        settings.humidity.high_range().end = 100.0;
        settings.relay_pins.dehumidifier = 2;

        // Test when humidity is in high range
        sd.set_average_humidity(70.0);
        handle_dehumidifier(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.dehumidifier),
            RelayStatus::On
        );

        // Test when humidity is not in high range
        sd.set_average_humidity(50.0);
        handle_dehumidifier(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.dehumidifier),
            RelayStatus::Off
        );
    }

    #[tokio::test]
    async fn test_handle_humidifier() {
        let sd = create_test_shared_data();
        let mut settings = Settings::new().unwrap();
        settings.humidity.low_range().start = 0.0;
        settings.humidity.low_range().end = 40.0;
        settings.humidity.humidifier_cooldown_duration = 0;
        settings.humidity.humidifier_activation_duration = 1;
        settings.relay_pins.humidifier = 3;

        // Test when humidity is in low range
        sd.set_average_humidity(30.0);
        handle_humidifier(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.humidifier),
            RelayStatus::Off
        );

        // Test when humidity is not in low range
        sd.set_average_humidity(50.0);
        handle_humidifier(sd.clone(), OffsetDateTime::now_utc(), settings.clone())
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.humidifier),
            RelayStatus::Off
        );
    }

    #[test]
    fn test_update_average_values() {
        let sd = create_test_shared_data();
        sd.set_temp_one(20.0);
        sd.set_temp_two(22.0);
        sd.set_humidity_one(50.0);
        sd.set_humidity_two(52.0);

        update_average_values(&sd);

        assert_eq!(sd.average_temp(), 21.0);
        assert_eq!(sd.average_humidity(), 51.0);
    }

    #[test]
    fn test_update_atmosphere_quality_index() {
        let sd = create_test_shared_data();
        let settings = Settings::new().unwrap();
        settings.temperature.ideal_range().start = 20.0;
        settings.temperature.ideal_range().end = 25.0;
        settings.humidity.ideal_range().start = 40.0;
        settings.humidity.ideal_range().end = 60.0;

        // Test when both temperature and humidity are in ideal range
        sd.set_average_temp(22.0);
        sd.set_average_humidity(50.0);
        update_atmosphere_quality_index(&sd, &settings);
        assert_eq!(sd.atmosphere_quality_index(), 100.0);

        // Test when either temperature or humidity is out of ideal range
        sd.set_average_temp(26.0);
        update_atmosphere_quality_index(&sd, &settings);
        assert_eq!(sd.atmosphere_quality_index(), 0.0);
    }
}
