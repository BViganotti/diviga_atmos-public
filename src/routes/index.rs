use crate::AccessSharedData;
use actix_web::{web, HttpResponse, Result};
use tera::Tera;
use time::format_description;

pub async fn index(
    common_data: web::Data<AccessSharedData>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse> {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

    let mut context = tera::Context::new();
    context.insert("average_temp", &common_data.average_temp());
    context.insert("average_humidity", &common_data.average_humidity());
    context.insert(
        "last_reading_time",
        &common_data.last_reading_datetime().format(&format).unwrap(),
    );
    context.insert("temp_one", &common_data.temp_one());
    context.insert("humidity_one", &common_data.humidity_one());
    context.insert("temp_two", &common_data.temp_two());
    context.insert("humidity_two", &common_data.humidity_two());
    context.insert("fridge_status", &common_data.fridge_status());
    context.insert(
        "fridge_turn_on_datetime",
        &common_data
            .fridge_turn_on_datetime()
            .format(&format)
            .unwrap(),
    );
    context.insert(
        "fridge_turn_off_datetime",
        &common_data
            .fridge_turn_off_datetime()
            .format(&format)
            .unwrap(),
    );
    context.insert("humidifier_status", &common_data.humidifier_status());
    context.insert(
        "humidifier_turn_on_datetime",
        &common_data
            .humidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
    );
    context.insert(
        "humidifier_turn_off_datetime",
        &common_data
            .humidifier_turn_off_datetime()
            .format(&format)
            .unwrap(),
    );
    context.insert("dehumidifier_status", &common_data.dehumidifier_status());
    context.insert(
        "dehumidifier_turn_on_datetime",
        &common_data
            .dehumidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
    );
    context.insert(
        "dehumidifier_turn_off_datetime",
        &common_data
            .dehumidifier_turn_off_datetime()
            .format(&format)
            .unwrap(),
    );

    let rendered = tmpl
        .render("index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
