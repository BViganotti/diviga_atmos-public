use actix_web::{web, App, HttpServer};

use crate::routes::atmosphere::get_atmosphere;
use crate::routes::get_full_atmospheric_data;
use crate::routes::heartbeat::pulse;
use crate::routes::index::index;
use crate::routes::relay_control::change_dehumidifier_status;
use crate::routes::relay_control::change_fridge_status;
use crate::routes::relay_control::trigger_humidifier;
use crate::routes::relay_status::get_dehumidifier_status;
use crate::routes::relay_status::get_fridge_status;
use crate::routes::relay_status::get_humidifier_status;
use crate::AccessSharedData;

pub async fn run_app(sd: &AccessSharedData) -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");
    let common_data = web::Data::new(sd.clone());

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .route("/get_atmosphere", web::get().to(get_atmosphere))
            .route(
                "/get_full_atmospheric_data",
                web::get().to(get_full_atmospheric_data),
            )
            .route(
                "/change_fridge_status",
                web::post().to(change_fridge_status),
            )
            .route("/trigger_humidifier", web::post().to(trigger_humidifier))
            .route(
                "/change_dehumidifier_status",
                web::post().to(change_dehumidifier_status),
            )
            .route("/get_fridge_status", web::get().to(get_fridge_status))
            .route(
                "/get_humidifier_status",
                web::get().to(get_humidifier_status),
            )
            .route(
                "/get_dehumidifier_status",
                web::get().to(get_dehumidifier_status),
            )
            .route("/heartbeat", web::get().to(pulse))
            .service(web::resource("/").to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run();

    server.await
}
