use crate::routes::atmosphere::get_atmosphere;
use crate::routes::atmosphere::get_atmosphere_history;
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
use crate::Arc;
use crate::Settings;
use crate::SqliteClient;
use actix_web::{web, App, HttpServer};
use log::info;

pub async fn run_app(
    sd: AccessSharedData,
    settings: Settings,
    mut shutdown_rx: tokio::sync::broadcast::Receiver<()>,
    sqlite_client: Arc<SqliteClient>,
) -> std::io::Result<()> {
    info!("Starting HTTP server at http://localhost:8080");

    let common_data = web::Data::new(sd);
    let common_settings = web::Data::new(settings);
    let common_sqlite_client = web::Data::new(sqlite_client);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(common_data.clone())
            .app_data(common_settings.clone())
            .app_data(common_sqlite_client.clone())
            .service(get_atmosphere)
            .service(get_full_atmospheric_data)
            .service(get_atmosphere_history)
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

    tokio::select! {
        result = server => {
            result?;
            Ok(())
        }
        _ = shutdown_rx.recv() => {
            println!("Webserver received shutdown signal");
            Ok(())
        }
    }
}
