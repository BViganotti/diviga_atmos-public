use crate::relay_ctrl::change_relay_status;
use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE,
};
use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};
use std::{thread, time::Duration};
use time::macros::offset;
use time::OffsetDateTime;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeFridgeStatus {
    previous_fridge_status: bool,
    new_fridge_status: bool,
    last_fridge_turn_on: String,
    last_fridge_turn_off: String,
    response: String,
}

pub async fn change_fridge_status(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let prev_fridge_status = sd.fridge_status();
    let mut res = String::new();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));

    if sd.fridge_status() == true {
        if now - sd.fridge_turn_on_datetime() < time::Duration::minutes(20) {
            // we might be just entering the ideal range so we also wait 20 minutes
            // because lowering the temp takes a while
            let wait_time = now - sd.fridge_turn_on_datetime();
            let how_long_to_wait = wait_time.as_seconds_f64() * 60.0;
            res = format!(
                "fridge still on, have to wait {} minutes before turn off",
                how_long_to_wait
            );
        } else {
            // more than 20 minutes have passed since the last turn on
            // we can safely turn off the fridge
            println!("fridge_control() -> turning off the fridge !");
            change_relay_status(RELAY_IN4_PIN_FRIDGE, false).expect("could not change relay");
            sd.set_fridge_status(false);
            sd.set_fridge_turn_off_datetime(now);
            res = "fridge turned off !".to_owned();
        }
    } else if sd.fridge_status() == false {
        if now - sd.fridge_turn_off_datetime() < time::Duration::minutes(20) {
            // we might be just entering the ideal range so we also wait 20 minutes
            // because lowering the temp takes a while
            let wait_time = now - sd.fridge_turn_off_datetime();
            let how_long_to_wait = wait_time.as_seconds_f64() * 60.0;
            res = format!(
                "fridge still off, have to wait {} minutes before turn on",
                how_long_to_wait
            );
        } else {
            // more than 20 minutes have passed since the last turn on
            // we can safely turn off the fridge
            println!("fridge_control() -> turning on the fridge !");
            change_relay_status(RELAY_IN4_PIN_FRIDGE, true).expect("could not change relay");
            sd.set_fridge_status(true);
            sd.set_fridge_turn_on_datetime(now);
            res = "fridge turned on !".to_owned();
        }
    }

    let values = ChangeFridgeStatus {
        previous_fridge_status: prev_fridge_status,
        new_fridge_status: sd.fridge_status(),
        last_fridge_turn_on: sd.fridge_turn_on_datetime().to_string(),
        last_fridge_turn_off: sd.fridge_turn_off_datetime().to_string(),
        response: res.to_owned(),
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeHumidifierStatus {
    humidifier_status: bool,
    last_humidifier_turn_on: String,
    last_humidifier_turn_off: String,
    response: String,
}

pub async fn trigger_humidifier(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let mut res = String::new();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));

    if sd.humidifier_status() != true {
        change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, true).expect("unable to change relay");
        sd.set_humidifier_status(true);
        sd.set_humidifier_turn_on_datetime(now);
        // in just a few seconds the humidity can reach 100% which isn't what i want
        // setting a sleep here and turning off the humidifer after a few seconds
        thread::sleep(Duration::from_secs(3));
        change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false).expect("unable to change relay");
        sd.set_humidifier_status(false);
        sd.set_humidifier_turn_off_datetime(now);
        res = "humidifier turned on and off for 3 secs".to_owned();
    }

    let values = ChangeHumidifierStatus {
        humidifier_status: sd.humidifier_status(),
        last_humidifier_turn_on: sd.humidifier_turn_on_datetime().to_string(),
        last_humidifier_turn_off: sd.humidifier_turn_off_datetime().to_string(),
        response: res.to_owned(),
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeDehumidifierStatus {
    previous_dehumidifier_status: bool,
    new_dehumidifier_status: bool,
    last_dehumidifier_turn_on: String,
    last_dehumidifier_turn_off: String,
    response: String,
}

pub async fn change_dehumidifier_status(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let mut res = String::new();
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    let prev_status = sd.dehumidifier_status();

    if sd.dehumidifier_status() != true {
        change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, true).expect("unable to change relay");
        sd.set_dehumidifier_status(true);
        sd.set_dehumidifier_turn_on_datetime(now);
        res = "dehumidifier turned on".to_owned();
    } else {
        change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false).expect("unable to change relay");
        sd.set_dehumidifier_status(false);
        sd.set_dehumidifier_turn_off_datetime(now);
        res = "dehumidifier turned off".to_owned();
    }

    let values = ChangeDehumidifierStatus {
        previous_dehumidifier_status: prev_status,
        new_dehumidifier_status: sd.dehumidifier_status(),
        last_dehumidifier_turn_on: sd.dehumidifier_turn_on_datetime().to_string(),
        last_dehumidifier_turn_off: sd.dehumidifier_turn_off_datetime().to_string(),
        response: res.to_owned(),
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeHeaterStatus {
    heater_status: bool,
    last_heater_turn_on: String,
    last_heater_turn_off: String,
    response: String,
}
