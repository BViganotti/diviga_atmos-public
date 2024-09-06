use crate::read_atmosphere;
use crate::shared_data::AccessSharedData;
use std::thread;
use std::time::Duration;

pub fn request_atmosphere(sd: &AccessSharedData) {
    loop {
        read_atmosphere::read_atmosphere_from_sensors(sd);
        thread::sleep(Duration::from_secs(45));
    }
}
