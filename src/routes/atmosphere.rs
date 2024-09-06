use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AvgAtmosphereData {
    temperature: f32,
    humidity: f32,
}

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
    fridge_status: bool,
    humidifier_status: bool,
    dehumidifier_status: bool,
    heater_status: bool,
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

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(values)
}
