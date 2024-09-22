use crate::error::AtmosError;
use crate::relay_ctrl::RelayStatus;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteClient {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteClient {
    pub fn new(path: &str) -> Result<Self, AtmosError> {
        let conn = Connection::open(path)?;
        let client = SqliteClient {
            conn: Arc::new(Mutex::new(conn)),
        };
        client.initialize_database()?;
        Ok(client)
    }

    fn initialize_database(&self) -> Result<(), AtmosError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS atmosphere_data (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                average_temperature REAL NOT NULL,
                average_humidity REAL NOT NULL,
                fridge_status TEXT NOT NULL,
                dehumidifier_status TEXT NOT NULL,
                humidifier_status TEXT NOT NULL,
                ventilator_status TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_atmosphere_data(
        &self,
        timestamp: OffsetDateTime,
        average_temperature: f32,
        average_humidity: f32,
        fridge_status: RelayStatus,
        dehumidifier_status: RelayStatus,
        humidifier_status: RelayStatus,
        ventilator_status: RelayStatus,
    ) -> Result<(), AtmosError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO atmosphere_data (
                timestamp, average_temperature, average_humidity, 
                fridge_status, dehumidifier_status, humidifier_status, ventilator_status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                timestamp.to_string(),
                average_temperature,
                average_humidity,
                fridge_status.to_string(),
                dehumidifier_status.to_string(),
                humidifier_status.to_string(),
                ventilator_status.to_string(),
            ],
        )?;
        Ok(())
    }
}
