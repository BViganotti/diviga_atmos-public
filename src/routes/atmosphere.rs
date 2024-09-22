use crate::{relay_ctrl::RelayStatus, sqlite_client::SqliteClient, AccessSharedData};
use actix_web::{get, http::header::ContentType, web, web::Query, HttpResponse};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AvgAtmosphereData {
    temperature: f32,
    humidity: f32,
}

#[get("/atmosphere")]
pub async fn get_atmosphere(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let values = AvgAtmosphereData {
        temperature: sd.average_temp(),
        humidity: sd.average_humidity(),
    };
    let values = serde_json::to_string(&values).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FullData {
    temp_1: f32,
    humidity_1: f32,
    temp_2: f32,
    humidity_2: f32,
    average_temp: f32,
    average_humidity: f32,
    atmospheric_quality_index: f32,
    fridge_status: RelayStatus,
    humidifier_status: RelayStatus,
    dehumidifier_status: RelayStatus,
    heater_status: RelayStatus,
    last_reading_time: String,
    fridge_turn_on_datetime: String,
    fridge_turn_off_datetime: String,
    humidifier_turn_on_datetime: String,
    humidifier_turn_off_datetime: String,
    dehumidifier_turn_on_datetime: String,
    dehumidifier_turn_off_datetime: String,
    heater_turn_on_datetime: String,
    heater_turn_off_datetime: String,
}

#[get("/api/atmosphere/full")]
pub async fn get_full_atmospheric_data(sd: web::Data<AccessSharedData>) -> HttpResponse {
    let values = FullData {
        temp_1: sd.temp_one(),
        humidity_1: sd.humidity_one(),
        temp_2: sd.temp_two(),
        humidity_2: sd.humidity_two(),
        average_temp: sd.average_temp(),
        average_humidity: sd.average_humidity(),
        atmospheric_quality_index: sd.atmosphere_quality_index(),
        fridge_status: sd.fridge_status(),
        humidifier_status: sd.humidifier_status(),
        dehumidifier_status: sd.dehumidifier_status(),
        heater_status: sd.heater_status(),
        last_reading_time: sd.last_reading_datetime().to_string(),
        fridge_turn_on_datetime: sd.fridge_turn_on_datetime().to_string(),
        fridge_turn_off_datetime: sd.fridge_turn_off_datetime().to_string(),
        humidifier_turn_on_datetime: sd.humidifier_turn_on_datetime().to_string(),
        humidifier_turn_off_datetime: sd.humidifier_turn_off_datetime().to_string(),
        dehumidifier_turn_on_datetime: sd.dehumidifier_turn_on_datetime().to_string(),
        dehumidifier_turn_off_datetime: sd.dehumidifier_turn_off_datetime().to_string(),
        heater_turn_on_datetime: sd.heater_turn_on_datetime().to_string(),
        heater_turn_off_datetime: sd.heater_turn_off_datetime().to_string(),
    };
    let values = serde_json::to_string(&values).unwrap();
    println!("Sending JSON response: {}", values);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}

#[get("/api/atmosphere/history")]
pub async fn get_atmosphere_history(
    sqlite_client: web::Data<SqliteClient>,
    query: Query<HashMap<String, String>>,
) -> HttpResponse {
    let default_range = "Today".to_string();
    let range = query.get("range").unwrap_or(&default_range);
    let limit = match range.as_str() {
        "Today" => 24,
        "Week" => 168,
        "Month" => 720,
        _ => 24,
    };

    match sqlite_client.read_atmosphere_data(limit) {
        Ok(json_data) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json_data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
