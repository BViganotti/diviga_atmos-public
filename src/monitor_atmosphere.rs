use crate::{
    error::AtmosError, relay_ctrl::change_relay_status, AccessSharedData, RelayStatus, Settings,
};
use log::{debug, info};
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

            // Handle devices sequentially
            handle_fridge(&sd, now, &settings).await?;
            handle_dehumidifier(&sd, now, &settings).await?;
            handle_humidifier(&sd, now, &settings).await?;
            handle_ventilator(&sd, now, &settings).await?;
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
    sd: &AccessSharedData,
    now: OffsetDateTime,
    settings: &Settings,
) -> Result<(), AtmosError> {
    let average_temp = sd.average_temp();
    if settings.temperature.high_range().contains(&average_temp) {
        let time_since_last_activation = now - sd.fridge_turn_off_datetime();
        if time_since_last_activation
            >= Duration::from_secs(settings.temperature.fridge_cooldown_duration)
        {
            info!("fridge_control() -> activating fridge");
            change_relay_status(settings.relay_pins.fridge, RelayStatus::On).await?;
            sd.set_fridge_status(RelayStatus::On);
            sd.set_fridge_turn_on_datetime(now);
        } else {
            info!("fridge_control() -> activation prevented due to cooldown period");
        }
    } else if settings.temperature.ideal_range().contains(&average_temp)
        || settings.temperature.low_range().contains(&average_temp)
    {
        if sd.fridge_status() == RelayStatus::On {
            info!("fridge_control() -> deactivating fridge");
            change_relay_status(settings.relay_pins.fridge, RelayStatus::Off).await?;
            sd.set_fridge_status(RelayStatus::Off);
            sd.set_fridge_turn_off_datetime(now);
        }
    }
    Ok(())
}

async fn handle_dehumidifier(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    settings: &Settings,
) -> Result<(), AtmosError> {
    let average_humidity = sd.average_humidity();
    if settings.humidity.high_range().contains(&average_humidity) {
        info!("dehumidifier_control() -> activating dehumidifier");
        change_relay_status(settings.relay_pins.dehumidifier, RelayStatus::On).await?;
        sd.set_dehumidifier_status(RelayStatus::On);
        sd.set_dehumidifier_turn_on_datetime(now);
    } else if !settings.humidity.high_range().contains(&average_humidity) {
        if sd.dehumidifier_status() == RelayStatus::On {
            info!("dehumidifier_control() -> deactivating dehumidifier");
            change_relay_status(settings.relay_pins.dehumidifier, RelayStatus::Off).await?;
            sd.set_dehumidifier_status(RelayStatus::Off);
            sd.set_dehumidifier_turn_off_datetime(now);
        }
    }
    Ok(())
}

async fn handle_humidifier(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    settings: &Settings,
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

async fn handle_ventilator(
    sd: &AccessSharedData,
    now: OffsetDateTime,
    settings: &Settings,
) -> Result<(), AtmosError> {
    if sd.ventilator_status() == RelayStatus::Off {
        let time_since_last_activation = now - sd.ventilator_turn_off_datetime();
        if time_since_last_activation >= Duration::from_secs(settings.ventilation.interval) {
            info!(
                "ventilator_control() -> activating ventilator for {} second(s)",
                settings.ventilation.duration
            );
            change_relay_status(settings.relay_pins.ventilator_or_heater, RelayStatus::On).await?;
            sd.set_ventilator_status(RelayStatus::On);
            sd.set_ventilator_turn_on_datetime(now);

            tokio::time::sleep(Duration::from_secs(settings.ventilation.duration)).await;

            info!("ventilator_control() -> deactivating ventilator");
            change_relay_status(settings.relay_pins.ventilator_or_heater, RelayStatus::Off).await?;
            sd.set_ventilator_status(RelayStatus::Off);
            sd.set_ventilator_turn_off_datetime(now);
        } else {
            info!("ventilator_control() -> activation prevented due to interval period");
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
        handle_fridge(&sd, OffsetDateTime::now_utc(), &settings)
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.fridge),
            RelayStatus::On
        );

        // Test when temperature is in ideal range
        sd.set_average_temp(22.0);
        handle_fridge(&sd, OffsetDateTime::now_utc(), &settings)
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
        handle_dehumidifier(&sd, OffsetDateTime::now_utc(), &settings)
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.dehumidifier),
            RelayStatus::On
        );

        // Test when humidity is not in high range
        sd.set_average_humidity(50.0);
        handle_dehumidifier(&sd, OffsetDateTime::now_utc(), &settings)
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
        handle_humidifier(&sd, OffsetDateTime::now_utc(), &settings)
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.humidifier),
            RelayStatus::Off
        );

        // Test when humidity is not in low range
        sd.set_average_humidity(50.0);
        handle_humidifier(&sd, OffsetDateTime::now_utc(), &settings)
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

    #[tokio::test]
    async fn test_handle_ventilator() {
        let sd = create_test_shared_data();
        let mut settings = Settings::new().unwrap();
        settings.ventilation.interval = 0;
        settings.ventilation.duration = 1;
        settings.relay_pins.ventilator_or_heater = 4;

        // Test ventilator activation
        handle_ventilator(&sd, OffsetDateTime::now_utc(), &settings)
            .await
            .unwrap();
        assert_eq!(
            get_mock_relay_status(settings.relay_pins.ventilator_or_heater),
            RelayStatus::Off
        );
        assert_eq!(sd.ventilator_status(), RelayStatus::Off);

        // Test ventilator not activating due to being already on
        sd.set_ventilator_status(RelayStatus::On);
        handle_ventilator(&sd, OffsetDateTime::now_utc(), &settings)
            .await
            .unwrap();
        assert_eq!(sd.ventilator_status(), RelayStatus::On);
    }
}
