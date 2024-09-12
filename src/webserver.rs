use crate::routes::atmosphere::get_atmosphere;
use crate::routes::get_full_atmospheric_data;
use crate::routes::heartbeat::pulse;
use crate::routes::relay_control::{
    change_dehumidifier_status, change_fridge_status, change_humidifier_status,
    change_ventilator_status,
};
use crate::routes::relay_status::{
    get_all_statuses, get_dehumidifier_status, get_fridge_status, get_humidifier_status,
    get_ventilator_status,
};
use crate::AccessSharedData;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use log::info;

pub fn run_app(sd: AccessSharedData) -> std::io::Result<Server> {
    info!("Starting HTTP server at http://localhost:8080");
    println!("starting HTTP server at http://localhost:8080");
    let common_data = web::Data::new(sd);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .service(get_atmosphere)
            .service(get_full_atmospheric_data)
            .service(pulse)
            .service(change_fridge_status)
            .service(change_humidifier_status)
            .service(change_dehumidifier_status)
            .service(change_ventilator_status)
            .service(get_all_statuses)
            .service(get_fridge_status)
            .service(get_humidifier_status)
            .service(get_dehumidifier_status)
            .service(get_ventilator_status)
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    Ok(server)
}
