use crate::config::Settings;
use crate::relay_ctrl::{self, RelayStatus};
use crate::shared_data::SharedData;
use time::macros::offset;
use time::OffsetDateTime;

pub fn initialize_shared_data() -> SharedData {
    SharedData::new(
        0,
        13.0,
        80.0,
        13.0,
        80.0,
        0.0,
        80.0,
        0.0,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        RelayStatus::Off,
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
        OffsetDateTime::UNIX_EPOCH.to_offset(offset!(+1)),
    )
}

pub async fn initialize_relay_pins(settings: &Settings) -> std::io::Result<()> {
    for pin in &[
        settings.relay_pins.humidifier,
        settings.relay_pins.dehumidifier,
        settings.relay_pins.ventilator_or_heater,
        settings.relay_pins.fridge,
    ] {
        relay_ctrl::change_relay_status(*pin, RelayStatus::Off)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    }
    Ok(())
}
