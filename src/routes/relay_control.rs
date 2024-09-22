use crate::config::Settings;
use crate::relay_ctrl::{change_relay_status, RelayStatus};
use crate::shared_data::AccessSharedData;
use actix_web::{post, web, HttpResponse};
use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize)]
struct RelayResponse {
    previous_status: RelayStatus,
    new_status: RelayStatus,
    last_turn_on: String,
    last_turn_off: String,
    response: String,
}

#[post("/change_fridge_status")]
pub async fn change_fridge_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let prev_status = sd.fridge_status();
    let new_status = if prev_status == RelayStatus::On {
        RelayStatus::Off
    } else {
        RelayStatus::On
    };

    let response = match change_relay_status(settings.relay_pins.fridge, new_status) {
        Ok(_) => {
            let now = OffsetDateTime::now_utc();
            sd.set_fridge_status(new_status);
            if new_status == RelayStatus::On {
                sd.set_fridge_turn_on_datetime(now);
            } else {
                sd.set_fridge_turn_off_datetime(now);
            }
            format!("Fridge turned {:?}", new_status)
        }
        Err(e) => format!("Error changing fridge status: {}", e),
    };

    let relay_response = RelayResponse {
        previous_status: prev_status,
        new_status: sd.fridge_status(),
        last_turn_on: sd.fridge_turn_on_datetime().to_string(),
        last_turn_off: sd.fridge_turn_off_datetime().to_string(),
        response,
    };

    HttpResponse::Ok().json(relay_response)
}

#[post("/change_humidifier_status")]
pub async fn change_humidifier_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let prev_status = sd.humidifier_status();
    let new_status = if prev_status == RelayStatus::On {
        RelayStatus::Off
    } else {
        RelayStatus::On
    };

    let response = match change_relay_status(settings.relay_pins.humidifier, new_status) {
        Ok(_) => {
            let now = OffsetDateTime::now_utc();
            sd.set_humidifier_status(new_status);
            if new_status == RelayStatus::On {
                sd.set_humidifier_turn_on_datetime(now);
            } else {
                sd.set_humidifier_turn_off_datetime(now);
            }
            format!("Humidifier turned {:?}", new_status)
        }
        Err(e) => format!("Error changing humidifier status: {}", e),
    };

    let relay_response = RelayResponse {
        previous_status: prev_status,
        new_status: sd.humidifier_status(),
        last_turn_on: sd.humidifier_turn_on_datetime().to_string(),
        last_turn_off: sd.humidifier_turn_off_datetime().to_string(),
        response,
    };

    HttpResponse::Ok().json(relay_response)
}

#[post("/change_dehumidifier_status")]
pub async fn change_dehumidifier_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let prev_status = sd.dehumidifier_status();
    let new_status = if prev_status == RelayStatus::On {
        RelayStatus::Off
    } else {
        RelayStatus::On
    };

    let response = match change_relay_status(settings.relay_pins.dehumidifier, new_status) {
        Ok(_) => {
            let now = OffsetDateTime::now_utc();
            sd.set_dehumidifier_status(new_status);
            if new_status == RelayStatus::On {
                sd.set_dehumidifier_turn_on_datetime(now);
            } else {
                sd.set_dehumidifier_turn_off_datetime(now);
            }
            format!("Dehumidifier turned {:?}", new_status)
        }
        Err(e) => format!("Error changing dehumidifier status: {}", e),
    };

    let relay_response = RelayResponse {
        previous_status: prev_status,
        new_status: sd.dehumidifier_status(),
        last_turn_on: sd.dehumidifier_turn_on_datetime().to_string(),
        last_turn_off: sd.dehumidifier_turn_off_datetime().to_string(),
        response,
    };

    HttpResponse::Ok().json(relay_response)
}

#[post("/change_ventilator_status")]
pub async fn change_ventilator_status(
    sd: web::Data<AccessSharedData>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let prev_status = sd.ventilator_status();
    let new_status = if prev_status == RelayStatus::On {
        RelayStatus::Off
    } else {
        RelayStatus::On
    };

    let response = match change_relay_status(settings.relay_pins.ventilator_or_heater, new_status) {
        Ok(_) => {
            let now = OffsetDateTime::now_utc();
            sd.set_ventilator_status(new_status);
            if new_status == RelayStatus::On {
                sd.set_ventilator_turn_on_datetime(now);
            } else {
                sd.set_ventilator_turn_off_datetime(now);
            }
            format!("Ventilator turned {:?}", new_status)
        }
        Err(e) => format!("Error changing ventilator status: {}", e),
    };

    let relay_response = RelayResponse {
        previous_status: prev_status,
        new_status: sd.ventilator_status(),
        last_turn_on: sd.ventilator_turn_on_datetime().to_string(),
        last_turn_off: sd.ventilator_turn_off_datetime().to_string(),
        response,
    };

    HttpResponse::Ok().json(relay_response)
}

// Similar functions for humidifier, dehumidifier, and ventilator
