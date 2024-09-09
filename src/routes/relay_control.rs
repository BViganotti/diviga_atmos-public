use crate::relay_ctrl::{
    change_relay_status as relay_status_change, RELAY_IN1_PIN_HUMIDIFIER,
    RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN4_PIN_FRIDGE,
};
use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Serialize;
//use std::{thread, time::Duration};
use time::{macros::offset, OffsetDateTime};
use tokio::time::Duration;

#[derive(Serialize)]
struct RelayResponse<T> {
    previous_status: bool,
    new_status: bool,
    last_turn_on: String,
    last_turn_off: String,
    response: String,
    #[serde(flatten)]
    extra: T,
}

async fn change_relay_status(
    sd: &web::Data<AccessSharedData>,
    relay_pin: u8,
    get_status: impl Fn(&AccessSharedData) -> bool,
    set_status: impl Fn(&AccessSharedData, bool),
    set_turn_on: impl Fn(&AccessSharedData, OffsetDateTime),
    set_turn_off: impl Fn(&AccessSharedData, OffsetDateTime),
    get_turn_on: impl Fn(&AccessSharedData) -> OffsetDateTime,
    get_turn_off: impl Fn(&AccessSharedData) -> OffsetDateTime,
    min_wait_time: Duration,
) -> RelayResponse<()> {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    let prev_status = get_status(sd);
    let response;

    if prev_status {
        if now - get_turn_on(sd) < min_wait_time {
            let wait_time = min_wait_time - (now - get_turn_on(sd));
            response = format!(
                "Still on, wait {:.1} minutes before turning off",
                wait_time.as_seconds_f64() / 60.0
            );
        } else {
            match relay_status_change(relay_pin, false).await {
                Ok(_) => {
                    set_status(sd, false);
                    set_turn_off(sd, now);
                    response = "Turned off".to_string();
                }
                Err(e) => {
                    response = format!("Error turning off: {}", e);
                }
            }
        }
    } else {
        if now - get_turn_off(sd) < min_wait_time {
            let wait_time = min_wait_time - (now - get_turn_off(sd));
            response = format!(
                "Still off, wait {:.1} minutes before turning on",
                wait_time.as_seconds_f64() / 60.0
            );
        } else {
            match relay_status_change(relay_pin, true).await {
                Ok(_) => {
                    set_status(sd, true);
                    set_turn_on(sd, now);
                    response = "Turned on".to_string();
                }
                Err(e) => {
                    response = format!("Error turning on: {}", e);
                }
            }
        }
    }

    RelayResponse {
        previous_status: prev_status,
        new_status: get_status(sd),
        last_turn_on: get_turn_on(sd).to_string(),
        last_turn_off: get_turn_off(sd).to_string(),
        response,
        extra: (),
    }
}

pub async fn change_fridge_status(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        RELAY_IN4_PIN_FRIDGE,
        AccessSharedData::fridge_status,
        AccessSharedData::set_fridge_status,
        AccessSharedData::set_fridge_turn_on_datetime,
        AccessSharedData::set_fridge_turn_off_datetime,
        AccessSharedData::fridge_turn_on_datetime,
        AccessSharedData::fridge_turn_off_datetime,
        Duration::from_secs(20 * 60),
    )
    .await;

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}

#[derive(Serialize)]
struct HumidifierResponse {
    humidifier_status: bool,
    last_humidifier_turn_on: String,
    last_humidifier_turn_off: String,
    response: String,
}

pub async fn trigger_humidifier(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));
    let mut response = String::new();

    if !sd.humidifier_status() {
        match relay_status_change(RELAY_IN1_PIN_HUMIDIFIER, true).await {
            Ok(_) => {
                sd.set_humidifier_status(true);
                sd.set_humidifier_turn_on_datetime(now);
                tokio::time::sleep(Duration::from_secs(1)).await;
                match relay_status_change(RELAY_IN1_PIN_HUMIDIFIER, false).await {
                    Ok(_) => {
                        sd.set_humidifier_status(false);
                        sd.set_humidifier_turn_off_datetime(now);
                        response = "Humidifier turned on and off for 1 sec".to_string();
                    }
                    Err(e) => {
                        response = format!("Error turning off humidifier: {}", e);
                    }
                }
            }
            Err(e) => {
                response = format!("Error turning on humidifier: {}", e);
            }
        }
    }

    let response = HumidifierResponse {
        humidifier_status: sd.humidifier_status(),
        last_humidifier_turn_on: sd.humidifier_turn_on_datetime().to_string(),
        last_humidifier_turn_off: sd.humidifier_turn_off_datetime().to_string(),
        response,
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}

#[derive(Serialize)]
struct DehumidifierResponse {
    previous_dehumidifier_status: bool,
    new_dehumidifier_status: bool,
    last_dehumidifier_turn_on: String,
    last_dehumidifier_turn_off: String,
    response: String,
}

pub async fn change_dehumidifier_status(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        RELAY_IN2_PIN_DEHUMIDIFIER,
        AccessSharedData::dehumidifier_status,
        AccessSharedData::set_dehumidifier_status,
        AccessSharedData::set_dehumidifier_turn_on_datetime,
        AccessSharedData::set_dehumidifier_turn_off_datetime,
        AccessSharedData::dehumidifier_turn_on_datetime,
        AccessSharedData::dehumidifier_turn_off_datetime,
        Duration::from_secs(20 * 60),
    )
    .await;

    let response = DehumidifierResponse {
        previous_dehumidifier_status: response.previous_status,
        new_dehumidifier_status: response.new_status,
        last_dehumidifier_turn_on: response.last_turn_on,
        last_dehumidifier_turn_off: response.last_turn_off,
        response: response.response,
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}

//#[derive(Serialize)]
//struct HeaterResponse {
//    heater_status: bool,
//    last_heater_turn_on: String,
//    last_heater_turn_off: String,
//    response: String,
//}
