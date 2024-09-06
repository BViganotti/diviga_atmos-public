use crate::config::Settings;
use crate::shared_data::AccessSharedData;
use log::{error, info, warn};
use std::{thread, time::Duration};

pub fn atmosphere_monitoring(sd: &AccessSharedData, settings: &Settings) {
    info!("Starting atmosphere monitoring");
    loop {
        update_average_values(sd);
        update_atmosphere_quality_index(sd, settings);

        if sd.polling_iterations() > 4 {
            control_environment(sd, settings);
        }

        info!(
            "Current atmosphere - Temp: {:.2}°C, Humidity: {:.2}%",
            sd.average_temp(),
            sd.average_humidity()
        );
        thread::sleep(Duration::from_secs(60));
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
    // Implement your control logic here
    if sd.average_temp() > settings.temperature.high_range_min {
        // Turn on cooling
        // For example: sd.set_fridge_status(true);
    } else if sd.average_temp() < settings.temperature.low_range_max {
        // Turn on heating
        // For example: sd.set_heater_status(true);
    } else {
        // Turn off both cooling and heating
        // For example:
        // sd.set_fridge_status(false);
        // sd.set_heater_status(false);
    }

    if sd.average_humidity() > settings.humidity.high_range_min {
        // Turn on dehumidifier
        // For example: sd.set_dehumidifier_status(true);
    } else if sd.average_humidity() < settings.humidity.low_range_max {
        // Turn on humidifier
        // For example: sd.set_humidifier_status(true);
    } else {
        // Turn off both dehumidifier and humidifier
        // For example:
        // sd.set_dehumidifier_status(false);
        // sd.set_humidifier_status(false);
    }
}

// ... rest of the functions ...

// Remove the duplicate control_environment function

// ... rest of the file remains the same ...
