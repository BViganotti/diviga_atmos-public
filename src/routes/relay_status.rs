use crate::{relay_ctrl::RelayStatus, AccessSharedData};
use actix_web::{get, http::header::ContentType, web, Error, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FridgeStatus {
    fridge_status: RelayStatus,
    last_fridge_turn_on: String,
    last_fridge_turn_off: String,
}

#[get("/get_fridge_status")]
pub async fn get_fridge_status(sd: web::Data<AccessSharedData>) -> Result<HttpResponse, Error> {
    let values = FridgeStatus {
        fridge_status: sd.fridge_status(),
        last_fridge_turn_on: sd.fridge_turn_on_datetime().to_string(),
        last_fridge_turn_off: sd.fridge_turn_off_datetime().to_string(),
    };
    let values = serde_json::to_string(&values)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HumidifierStatus {
    humidifier_status: RelayStatus,
    last_humidifier_turn_on: String,
    last_humidifier_turn_off: String,
}

#[get("/get_humidifier_status")]
pub async fn get_humidifier_status(sd: web::Data<AccessSharedData>) -> Result<HttpResponse, Error> {
    let values = HumidifierStatus {
        humidifier_status: sd.humidifier_status(),
        last_humidifier_turn_on: sd.humidifier_turn_on_datetime().to_string(),
        last_humidifier_turn_off: sd.humidifier_turn_off_datetime().to_string(),
    };
    let values = serde_json::to_string(&values)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DehumidifierStatus {
    dehumidifier_status: RelayStatus,
    last_dehumidifier_turn_on: String,
    last_dehumidifier_turn_off: String,
}
#[get("/get_dehumidifier_status")]
pub async fn get_dehumidifier_status(
    sd: web::Data<AccessSharedData>,
) -> Result<HttpResponse, Error> {
    let values = DehumidifierStatus {
        dehumidifier_status: sd.dehumidifier_status(),
        last_dehumidifier_turn_on: sd.dehumidifier_turn_on_datetime().to_string(),
        last_dehumidifier_turn_off: sd.dehumidifier_turn_off_datetime().to_string(),
    };
    let values = serde_json::to_string(&values)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VentilatorStatus {
    ventilator_status: RelayStatus,
    last_ventilator_turn_on: String,
    last_ventilator_turn_off: String,
}

#[get("/get_ventilator_status")]
pub async fn get_ventilator_status(sd: web::Data<AccessSharedData>) -> Result<HttpResponse, Error> {
    let values = VentilatorStatus {
        ventilator_status: sd.ventilator_status(),
        last_ventilator_turn_on: sd.ventilator_turn_on_datetime().to_string(),
        last_ventilator_turn_off: sd.ventilator_turn_off_datetime().to_string(),
    };
    let values = serde_json::to_string(&values)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values))
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AllStatuses {
    fridge_status: RelayStatus,
    humidifier_status: RelayStatus,
    dehumidifier_status: RelayStatus,
    heater_status: RelayStatus,
    ventilator_status: RelayStatus,
}

#[get("/get_all_statuses")]
pub async fn get_all_statuses(sd: web::Data<AccessSharedData>) -> Result<HttpResponse, Error> {
    let values = AllStatuses {
        fridge_status: sd.fridge_status(),
        humidifier_status: sd.humidifier_status(),
        dehumidifier_status: sd.dehumidifier_status(),
        heater_status: sd.heater_status(),
        ventilator_status: sd.ventilator_status(),
    };
    let values = serde_json::to_string(&values)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values))
}
