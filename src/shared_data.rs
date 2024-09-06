use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

// A struct to hold the values that will be shared across all threads in the application
pub struct SharedData {
    polling_iterations: u64,
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
    last_reading_time: OffsetDateTime,
    fridge_turn_on_datetime: OffsetDateTime,
    fridge_turn_off_datetime: OffsetDateTime,
    humidifier_turn_on_datetime: OffsetDateTime,
    humidifier_turn_off_datetime: OffsetDateTime,
    dehumidifier_turn_on_datetime: OffsetDateTime,
    dehumidifier_turn_off_datetime: OffsetDateTime,
    heater_turn_on_datetime: OffsetDateTime,
    heater_turn_off_datetime: OffsetDateTime,
}

impl SharedData {
    pub fn new(
        polling_iterations: u64,
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
        last_reading_time: OffsetDateTime,
        fridge_turn_on_datetime: OffsetDateTime,
        fridge_turn_off_datetime: OffsetDateTime,
        humidifier_turn_on_datetime: OffsetDateTime,
        humidifier_turn_off_datetime: OffsetDateTime,
        dehumidifier_turn_on_datetime: OffsetDateTime,
        dehumidifier_turn_off_datetime: OffsetDateTime,
        heater_turn_on_datetime: OffsetDateTime,
        heater_turn_off_datetime: OffsetDateTime,
    ) -> SharedData {
        SharedData {
            polling_iterations,
            temp_1,
            humidity_1,
            temp_2,
            humidity_2,
            average_temp,
            average_humidity,
            atmospheric_quality_index,
            fridge_status,
            humidifier_status,
            dehumidifier_status,
            heater_status,
            last_reading_time,
            fridge_turn_on_datetime,
            fridge_turn_off_datetime,
            humidifier_turn_on_datetime,
            humidifier_turn_off_datetime,
            dehumidifier_turn_on_datetime,
            dehumidifier_turn_off_datetime,
            heater_turn_on_datetime,
            heater_turn_off_datetime,
        }
    }
}

// The struct that will be used to manage access to the shared data struct.
pub struct AccessSharedData {
    pub sd: Arc<Mutex<SharedData>>,
}

// Clone here just makes a copy of the Arc pointer - not  the entire class of data
// All clones point to the same internal data
impl Clone for AccessSharedData {
    fn clone(&self) -> Self {
        AccessSharedData {
            sd: Arc::clone(&self.sd),
        }
    }
}

// Getters/Setters for access to the shared data. Everything is wrapped in a MutexGuard to
// ensure thread safety for every access point.
impl AccessSharedData {
    pub fn polling_iterations(&self) -> u64 {
        let lock = self.sd.lock().unwrap();
        lock.polling_iterations
    }
    pub fn increment_polling_iterations(&self) {
        let mut lock = self.sd.lock().unwrap();
        lock.polling_iterations = lock.polling_iterations + 1;
    }

    pub fn temp_one(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.temp_1
    }
    pub fn set_temp_one(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.temp_1 = new_val;
    }

    pub fn humidity_one(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.humidity_1
    }
    pub fn set_humidity_one(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.humidity_1 = new_val;
    }

    pub fn temp_two(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.temp_2
    }
    pub fn set_temp_two(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.temp_2 = new_val;
    }

    pub fn humidity_two(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.humidity_2
    }
    pub fn set_humidity_two(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.humidity_2 = new_val;
    }

    pub fn average_temp(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.average_temp
    }
    pub fn set_average_temp(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.average_temp = new_val;
    }

    pub fn average_humidity(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.average_humidity
    }
    pub fn set_average_humidity(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.average_humidity = new_val;
    }

    pub fn atmosphere_quality_index(&self) -> f32 {
        let lock = self.sd.lock().unwrap();
        lock.atmospheric_quality_index
    }
    pub fn set_atmosphere_quality_index(&self, new_val: f32) {
        let mut lock = self.sd.lock().unwrap();
        lock.atmospheric_quality_index = new_val;
    }

    pub fn fridge_status(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.fridge_status
    }
    pub fn set_fridge_status(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.fridge_status = new_val;
    }

    pub fn humidifier_status(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.humidifier_status
    }
    pub fn set_humidifier_status(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.humidifier_status = new_val;
    }
    pub fn dehumidifier_status(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.dehumidifier_status
    }
    pub fn set_dehumidifier_status(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.dehumidifier_status = new_val;
    }
    pub fn heater_status(&self) -> bool {
        let lock = self.sd.lock().unwrap();
        lock.heater_status
    }
    pub fn set_heater_status(&self, new_val: bool) {
        let mut lock = self.sd.lock().unwrap();
        lock.heater_status = new_val;
    }

    pub fn last_reading_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.last_reading_time
    }
    pub fn set_last_reading_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.last_reading_time = dt;
    }

    pub fn fridge_turn_on_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.fridge_turn_on_datetime
    }
    pub fn set_fridge_turn_on_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.fridge_turn_on_datetime = dt;
    }

    pub fn fridge_turn_off_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.fridge_turn_off_datetime
    }
    pub fn set_fridge_turn_off_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.fridge_turn_off_datetime = dt;
    }

    pub fn humidifier_turn_on_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.humidifier_turn_on_datetime
    }
    pub fn set_humidifier_turn_on_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.humidifier_turn_on_datetime = dt;
    }

    pub fn humidifier_turn_off_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.humidifier_turn_off_datetime
    }
    pub fn set_humidifier_turn_off_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.humidifier_turn_off_datetime = dt;
    }

    pub fn dehumidifier_turn_on_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.dehumidifier_turn_on_datetime
    }
    pub fn set_dehumidifier_turn_on_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.dehumidifier_turn_on_datetime = dt;
    }

    pub fn dehumidifier_turn_off_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.dehumidifier_turn_off_datetime
    }
    pub fn set_dehumidifier_turn_off_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.dehumidifier_turn_off_datetime = dt;
    }

    pub fn heater_turn_on_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.heater_turn_on_datetime
    }
    pub fn set_heater_turn_on_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.heater_turn_on_datetime = dt;
    }

    pub fn heater_turn_off_datetime(&self) -> OffsetDateTime {
        let lock = self.sd.lock().unwrap();
        lock.heater_turn_off_datetime
    }
    pub fn set_heater_turn_off_datetime(&self, dt: OffsetDateTime) {
        let mut lock = self.sd.lock().unwrap();
        lock.heater_turn_off_datetime = dt;
    }
}
