use crate::config::Settings;
use crate::routes::atmosphere::get_atmosphere;
use crate::routes::get_full_atmospheric_data;
use crate::routes::heartbeat::pulse;
use crate::routes::index::index;
use crate::routes::relay_status::get_dehumidifier_status;
use crate::routes::relay_status::get_fridge_status;
use crate::routes::relay_status::get_humidifier_status;
use crate::AccessSharedData;
use actix_web::{web, App, HttpServer};
use log::info;
use tera::Tera;

pub async fn run_app(sd: &AccessSharedData, settings: &Settings) -> std::io::Result<()> {
    info!("Starting HTTP server at http://localhost:8080");
    println!("starting HTTP server at http://localhost:8080");
    let common_data = web::Data::new(sd.clone());

    let tera = Tera::new("templates/**/*")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let tera_data = web::Data::new(tera);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .app_data(tera_data.clone())
            .service(
                web::scope("/api")
                    .route("/atmosphere", web::get().to(get_atmosphere))
                    .route("/atmosphere/full", web::get().to(get_full_atmospheric_data))
                    .service(
                        web::scope("/devices")
                            .route("/fridge", web::get().to(get_fridge_status))
                            .route("/humidifier", web::get().to(get_humidifier_status))
                            .route("/dehumidifier", web::get().to(get_dehumidifier_status)),
                    ),
            )
            .route("/", web::get().to(index))
            .route("/heartbeat", web::get().to(pulse))
    })
    .bind((settings.webserver.host.clone(), settings.webserver.port))?
    .workers(2)
    .run();

    server.await
}
