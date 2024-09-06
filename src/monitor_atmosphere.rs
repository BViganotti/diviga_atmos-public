use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE,
};
// RELAY_IN3_PIN_VENTILATOR_OR_HEATER is for the heater, i don't know if i need one yet
use crate::{relay_ctrl, shared_data::AccessSharedData};
use std::{thread, time::Duration};
use time::format_description;
use time::macros::offset;
use time::OffsetDateTime;

const LOW_TEMPERATURE_RANGE: std::ops::Range<f32> = -20.0..11.0;
const HIGH_TEMPERATURE_RANGE: std::ops::Range<f32> = 14.0..100.0;
const IDEAL_TEMPERATURE_RANGE: std::ops::Range<f32> = 11.0..14.0;

const LOW_HUMIDITY_RANGE: std::ops::Range<f32> = 0.0..60.0;
const HIGH_HUMIDITY_RANGE: std::ops::Range<f32> = 80.0..100.0;
const IDEAL_HUMIDITY_RANGE: std::ops::Range<f32> = 75.0..80.00;

pub fn atmosphere_monitoring(sd: &AccessSharedData) {
    loop {
        average_temperature(&sd);
        average_humidity(&sd);
        atmosphere_quality_index(&sd);

        // the first 4 iterations the sensors often are badly calibrated
        // which results in crazy values like -50 degrees, i want to wait
        // for better data before triggering any relays.
        if sd.polling_iterations() > 4 {
            fridge_control(&sd);
            //humidifier_control(&sd);
            dehumidifier_control(&sd);
        }

        debug_data_display(&sd);

        thread::sleep(Duration::from_secs(60));
    }
}

fn fridge_control(sd: &AccessSharedData) {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    if HIGH_TEMPERATURE_RANGE.contains(&sd.average_temp()) {
        println!("fridge_control() -> high temp range");
        if sd.fridge_status() == false {
            if now - sd.fridge_turn_off_datetime() < time::Duration::minutes(15) {
                // we wait and do nothing, we don't want to burn the compressor
                let wait_time = now - sd.fridge_turn_on_datetime();
                println!(
                    "fridge_control() -> waiting {} minutes",
                    wait_time.as_seconds_f64() * 60.0
                );
            } else {
                // more than 15 minutes have passed since the last turn off
                // we can safely turn on the fridge
                println!("fridge_control() -> turning on the fridge !");
                relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, true)
                    .expect("unable to change relay");
                sd.set_fridge_status(true);
                sd.set_fridge_turn_on_datetime(now);
            }
        }
    // else we don't do anything, the fridge is on
    } else if IDEAL_TEMPERATURE_RANGE.contains(&sd.average_temp()) {
        println!("fridge_control() -> ideal temp range");
        if sd.fridge_status() == true {
            if now - sd.fridge_turn_on_datetime() < time::Duration::minutes(30) {
                // we might be just entering the ideal range so we also wait 30 minutes
                // because lowering the temp takes a while
                let wait_time = now - sd.fridge_turn_on_datetime();
                println!(
                    "fridge_control() -> waiting {} minutes",
                    wait_time.as_seconds_f64() * 60.0
                );
            } else {
                // more than 30 minutes have passed since the last turn on
                // we can safely turn off the fridge
                println!("fridge_control() -> turning off the fridge !");
                relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, false)
                    .expect("unable to change relay");
                sd.set_fridge_status(false);
                sd.set_fridge_turn_off_datetime(now);
            }
        }
    } else if LOW_TEMPERATURE_RANGE.contains(&sd.average_temp()) {
        println!("fridge_control() -> low temp range");
        if sd.fridge_status() == true {
            if now - sd.fridge_turn_on_datetime() < time::Duration::minutes(20) {
                // we might be just entering the ideal range so we also wait 20 minutes
                // because lowering the temp takes a while
                let wait_time = now - sd.fridge_turn_on_datetime();
                println!(
                    "fridge_control() -> waiting {} minutes",
                    wait_time.as_seconds_f64() * 60.0
                );
            } else {
                // more than 0 minutes have passed since the last turn on
                // we can safely turn off the fridge
                println!("fridge_control() -> turning off the fridge !");
                relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, false)
                    .expect("unable to change relay");
                sd.set_fridge_status(false);
                sd.set_fridge_turn_off_datetime(now);
            }
        }
    }
}

fn humidifier_control(sd: &AccessSharedData) {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    if LOW_HUMIDITY_RANGE.contains(&sd.average_humidity()) {
        println!("humidifier_control() -> low humidity range");
        if sd.humidifier_status() != true {
            println!("humidifier_control() -> turning on humidifier !");
            relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, true)
                .expect("unable to change relay");
            sd.set_humidifier_status(true);
            sd.set_humidifier_turn_on_datetime(now);
            // in just a few seconds the humidity can reach 100% which isn't what i want
            // setting a sleep here and turning off the humidifer after a few seconds
            thread::sleep(Duration::from_secs(3));
            println!("humidifier_control() -> 3secs passed ! turning off humidifier !");
            relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false)
                .expect("unable to change relay");
            sd.set_humidifier_status(false);
            sd.set_humidifier_turn_off_datetime(now);
        }
    } else if IDEAL_HUMIDITY_RANGE.contains(&sd.average_humidity()) {
        println!("humidifier_control() -> ideal humidity range");
        if sd.humidifier_status() == true {
            println!("humidifier_control() -> turning off humidifier !");
            relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false)
                .expect("unable to change relay");
            sd.set_humidifier_status(false);
            sd.set_humidifier_turn_off_datetime(now);
        }
    }
}

fn dehumidifier_control(sd: &AccessSharedData) {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    if HIGH_HUMIDITY_RANGE.contains(&sd.average_humidity()) {
        println!("dehumidifier_control() -> high humidity range");
        if sd.dehumidifier_status() != true {
            println!("dehumidifier_control() -> turning on dehumidifier");
            relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, true)
                .expect("unable to change relay");
            sd.set_dehumidifier_status(true);
            sd.set_dehumidifier_turn_on_datetime(now);
        }
    } else if IDEAL_HUMIDITY_RANGE.contains(&sd.average_humidity()) {
        println!("dehumidifier_control() -> ideal humidity range");
        if sd.dehumidifier_status() == true {
            println!("dehumidifier_control() -> turning off dehumidifier !");
            relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false)
                .expect("unable to change relay");
            sd.set_dehumidifier_status(false);
            sd.set_dehumidifier_turn_off_datetime(now);
        }
    } else if LOW_HUMIDITY_RANGE.contains(&sd.average_humidity()) {
        println!("dehumidifier_control() -> low humidity range");
        if sd.dehumidifier_status() == true {
            println!("dehumidifier_control() -> turning off dehumidifier !");
            relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false)
                .expect("unable to change relay");
            sd.set_dehumidifier_status(false);
            sd.set_dehumidifier_turn_off_datetime(now);
        }
    }
}

fn _heater_control(_sd: &AccessSharedData) {}

pub fn average_temperature(sd: &AccessSharedData) {
    let average_temp = (sd.temp_one() + sd.temp_two()) / 2.;

    sd.set_average_temp(average_temp);
}

pub fn average_humidity(sd: &AccessSharedData) {
    let average_humidity = (sd.humidity_one() + sd.humidity_two()) / 2.;

    sd.set_average_humidity(average_humidity);
}

pub fn atmosphere_quality_index(sd: &AccessSharedData) {
    let temp_range: std::ops::Range<f32> = 11.0..15.0;
    let humidity_range: std::ops::Range<f32> = 76.0..83.0;

    if temp_range.contains(&sd.average_temp()) && humidity_range.contains(&sd.average_humidity()) {
        sd.set_atmosphere_quality_index(100.0);
    } else {
        sd.set_atmosphere_quality_index(0.0);
    }
}

pub fn debug_data_display(sd: &AccessSharedData) {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    println!(
        "polling iterations: {}
last reading time: {}
temp1: {} - humidity1: {}
temp2: {} - humidity2: {}
average temperature: {}
average humidity: {}
atmospheric quality index: {}
fridge status: {}
humidifier status: {}
dehumidifer status: {}
heater status: {}
last fridge turn on time: {}
last fridge turn off time: {}
last humidifer turn on time: {}
last humidifer turn off time: {}
last dehumidifier turn on time: {}
last dehumidifier turn off time: {}
last heater turn on time: {}
last heater turn off time: {}\n",
        sd.polling_iterations(),
        sd.last_reading_datetime().format(&format).unwrap(),
        sd.temp_one(),
        sd.humidity_one(),
        sd.temp_two(),
        sd.humidity_two(),
        sd.average_temp(),
        sd.average_humidity(),
        sd.atmosphere_quality_index(),
        sd.fridge_status(),
        sd.humidifier_status(),
        sd.dehumidifier_status(),
        sd.heater_status(),
        sd.fridge_turn_on_datetime().format(&format).unwrap(),
        sd.fridge_turn_off_datetime().format(&format).unwrap(),
        sd.humidifier_turn_on_datetime().format(&format).unwrap(),
        sd.humidifier_turn_off_datetime().format(&format).unwrap(),
        sd.dehumidifier_turn_on_datetime().format(&format).unwrap(),
        sd.dehumidifier_turn_off_datetime().format(&format).unwrap(),
        sd.heater_turn_on_datetime().format(&format).unwrap(),
        sd.heater_turn_off_datetime().format(&format).unwrap()
    );
}

/*
dupa
*/
