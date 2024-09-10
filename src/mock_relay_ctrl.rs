use crate::error::AtmosError;
use crate::relay_ctrl::RelayStatus;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

lazy_static! {
    static ref MOCK_RELAY_STATES: Mutex<HashMap<u8, AtomicBool>> = Mutex::new(HashMap::new());
}

pub async fn mock_change_relay_status(pin: u8, status: RelayStatus) -> Result<(), AtmosError> {
    let mut states = MOCK_RELAY_STATES.lock().unwrap();
    let state = states.entry(pin).or_insert_with(|| AtomicBool::new(false));
    state.store(status == RelayStatus::On, Ordering::SeqCst);
    Ok(())
}

pub fn get_mock_relay_status(pin: u8) -> RelayStatus {
    let states = MOCK_RELAY_STATES.lock().unwrap();
    if let Some(state) = states.get(&pin) {
        if state.load(Ordering::SeqCst) {
            RelayStatus::On
        } else {
            RelayStatus::Off
        }
    } else {
        RelayStatus::Off
    }
}
