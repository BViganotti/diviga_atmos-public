use crate::config::Settings;
use crate::relay_ctrl::{self, RelayStatus};
use crate::shared_data::SharedData;
use log::{error, info};
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
    info!("Starting relay pin initialization");
    for pin in &[
        settings.relay_pins.humidifier,
        settings.relay_pins.dehumidifier,
        settings.relay_pins.ventilator_or_heater,
        settings.relay_pins.fridge,
    ] {
        info!("Initializing relay pin {}", pin);
        let current_status = match relay_ctrl::check_relay_status(*pin) {
            Ok(status) => {
                info!("Current status of pin {} is {:?}", pin, status);
                status
            }
            Err(e) => {
                error!("Failed to check status of pin {}: {}", pin, e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ));
            }
        };

        if current_status == RelayStatus::On {
            info!("Pin {} is On, attempting to turn it Off", pin);
            match relay_ctrl::change_relay_status(*pin, RelayStatus::Off) {
                Ok(_) => info!("Successfully turned off pin {}", pin),
                Err(e) => {
                    error!("Failed to turn off pin {}: {}", pin, e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    ));
                }
            }
        } else {
            info!("Pin {} is already Off, no action needed", pin);
        }
    }
    info!("Relay pin initialization completed successfully");
    Ok(())
}

pub async fn deinitialize_relay_pins(
    settings: &Settings,
    mut rx: tokio::sync::broadcast::Receiver<()>,
) -> std::io::Result<()> {
    info!("Waiting for shutdown signal");
    rx.recv().await.unwrap();
    info!("Shutdown signal received");
    info!("Starting relay pin deinitialization");
    for pin in &[
        settings.relay_pins.humidifier,
        settings.relay_pins.dehumidifier,
        settings.relay_pins.ventilator_or_heater,
        settings.relay_pins.fridge,
    ] {
        info!("Deinitializing relay pin {}", pin);
        match relay_ctrl::change_relay_status(*pin, RelayStatus::Off) {
            Ok(_) => info!("Successfully turned off pin {}", pin),
            Err(e) => {
                error!("Failed to turn off pin {}: {}", pin, e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ));
            }
        }
    }
    info!("Relay pin deinitialization completed successfully");
    Ok(())
}
