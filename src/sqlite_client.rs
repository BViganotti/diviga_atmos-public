use crate::error::AtmosError;
use crate::relay_ctrl::RelayStatus;
use rusqlite::{params, Connection, Result, Row};
use serde_json::json;
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

    pub fn read_atmosphere_data(&self, limit: usize) -> Result<String, AtmosError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT timestamp, average_temperature, average_humidity, 
             fridge_status, dehumidifier_status, humidifier_status, ventilator_status 
             FROM atmosphere_data ORDER BY timestamp DESC LIMIT ?",
        )?;

        let rows = stmt.query_map([limit], |row: &Row| {
            Ok(json!({
                "timestamp": row.get::<_, String>(0)?,
                "average_temperature": row.get::<_, f32>(1)?,
                "average_humidity": row.get::<_, f32>(2)?,
                "fridge_status": row.get::<_, String>(3)?,
                "dehumidifier_status": row.get::<_, String>(4)?,
                "humidifier_status": row.get::<_, String>(5)?,
                "ventilator_status": row.get::<_, String>(6)?,
            }))
        })?;

        let data: Vec<serde_json::Value> = rows.collect::<Result<_, _>>()?;
        Ok(serde_json::to_string(&data)?)
    }
}
