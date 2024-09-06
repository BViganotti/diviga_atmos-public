/*
This project is very very much influenced and heavily copy pasted from https://github.com/mikehentges/thermostat-pi.
*/

pub mod monitor_atmosphere;
pub mod read_atmosphere;
pub mod relay_ctrl;
pub mod request_atmosphere;
pub mod routes;
pub mod shared_data;
pub mod ventilation;
pub mod webserver;

use crate::relay_ctrl::{
    RELAY_IN1_PIN_HUMIDIFIER, RELAY_IN2_PIN_DEHUMIDIFIER, RELAY_IN3_PIN_VENTILATOR_OR_HEATER,
    RELAY_IN4_PIN_FRIDGE,
};
use crate::shared_data::AccessSharedData;
use crate::shared_data::SharedData;
use actix_web::rt;
use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use time::macros::offset;
use time::OffsetDateTime;

fn main() {
    dotenv().ok();

    // Initialize a struct that will be our "global" data, which allows safe access from every thread
    let common_data = SharedData::new(
        0,
        13.0,
        80.0,
        13.0,
        80.0,
        0.0,
        80.0,
        0.0,
        false,
        false,
        false,
        false,
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
    );

    // setting all the pins to false just in case
    relay_ctrl::change_relay_status(RELAY_IN1_PIN_HUMIDIFIER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN2_PIN_DEHUMIDIFIER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN3_PIN_VENTILATOR_OR_HEATER, false)
        .expect("unable to initialize relay");
    relay_ctrl::change_relay_status(RELAY_IN4_PIN_FRIDGE, false)
        .expect("unable to initialize relay");

    // The wrapper around our shared data that gives it safe access across threads
    let sd = AccessSharedData {
        sd: Arc::new(Mutex::new(common_data)),
    };

    // We are cloning the pointer to our shared data, and sending it into
    // a new thread that continuously reads the temperature from our sensor,
    // and updates the SharedData::current_temp value.
    let sdclone_1 = sd.clone();

    let handle = thread::spawn(move || {
        request_atmosphere::request_atmosphere(&sdclone_1);
    });

    thread::spawn(move || {
        ventilation::ventilation_loop();
    });

    let sdclone_2 = sd.clone();
    thread::spawn(move || {
        monitor_atmosphere::atmosphere_monitoring(&sdclone_2);
    });

    let sdclone_3 = sd.clone();
    thread::spawn(move || {
        let server_future = webserver::run_app(&sdclone_3);
        rt::System::new().block_on(server_future)
    });

    let _ = handle.join();
}
