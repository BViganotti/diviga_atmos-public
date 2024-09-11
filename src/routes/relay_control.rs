use crate::config::Settings;
use crate::AccessSharedData;
use actix_web::{post, web, HttpResponse};
use serde::Serialize;
use time::{Duration, OffsetDateTime};

#[derive(Serialize)]
struct RelayResponse {
    previous_status: bool,
    new_status: bool,
    last_turn_on: String,
    last_turn_off: String,
    response: String,
}

async fn change_relay_status(
    sd: &AccessSharedData,
    settings: &Settings,
    relay_pin: u8,
    min_wait_time: Duration,
) -> RelayResponse {
    let now = OffsetDateTime::now_utc();
    let prev_status = sd.get_relay_status(relay_pin);
    let new_status = !prev_status;

    let response = if (new_status && now - sd.get_last_turn_off(relay_pin) < min_wait_time)
        || (!new_status && now - sd.get_last_turn_on(relay_pin) < min_wait_time)
    {
        let wait_time = min_wait_time
            - (now
                - if new_status {
                    sd.get_last_turn_off(relay_pin)
                } else {
                    sd.get_last_turn_on(relay_pin)
                });
        format!(
            "Still {}, wait {:.1} minutes before turning {}",
            if new_status { "off" } else { "on" },
            wait_time.as_seconds_f64() / 60.0,
            if new_status { "on" } else { "off" }
        )
    } else {
        match crate::relay_ctrl::change_relay_status(relay_pin, new_status).await {
            Ok(_) => {
                sd.set_relay_status(relay_pin, new_status);
                if new_status {
                    sd.set_last_turn_on(relay_pin, now);
                } else {
                    sd.set_last_turn_off(relay_pin, now);
                }
                format!("Turned {}", if new_status { "on" } else { "off" })
            }
            Err(e) => format!("Error changing status: {}", e),
        }
    };

    RelayResponse {
        previous_status: prev_status,
        new_status: sd.get_relay_status(relay_pin),
        last_turn_on: sd.get_last_turn_on(relay_pin).to_string(),
        last_turn_off: sd.get_last_turn_off(relay_pin).to_string(),
        response,
    }
}

#[post("/change_fridge_status")]
pub async fn change_fridge_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        &settings,
        settings.relay_pins.fridge,
        Duration::minutes(settings.fridge.min_off_time as i64),
    )
    .await;

    HttpResponse::Ok().json(response)
}

#[post("/change_humidifier_status")]
pub async fn change_humidifier_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        &settings,
        settings.relay_pins.humidifier,
        Duration::minutes(settings.humidifier.min_off_time as i64),
    )
    .await;

    HttpResponse::Ok().json(response)
}

#[post("/change_dehumidifier_status")]
pub async fn change_dehumidifier_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        &settings,
        settings.relay_pins.dehumidifier,
        Duration::minutes(settings.dehumidifier.min_off_time as i64),
    )
    .await;

    HttpResponse::Ok().json(response)
}

#[post("/change_ventilator_status")]
pub async fn change_ventilator_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let response = change_relay_status(
        &sd,
        &settings,
        settings.relay_pins.ventilator,
        Duration::minutes(settings.ventilator.min_off_time as i64),
    )
    .await;

    HttpResponse::Ok().json(response)
}
