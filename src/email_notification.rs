use crate::error::AtmosError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email_notification(
    config: &EmailConfig,
    subject: &str,
    body: &str,
) -> Result<(), AtmosError> {
    let email = Message::builder()
        .from(config.from_address.parse().unwrap())
        .to(config.to_address.parse().unwrap())
        .subject(subject)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let mailer = SmtpTransport::relay(&config.smtp_server)
        .unwrap()
        .credentials(creds)
        .port(config.smtp_port)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(AtmosError::EmailError(e.to_string())),
    }
}

pub fn create_alert_message(alert_type: &str, details: &str) -> (String, String) {
    let subject = format!("Atmos Alert: {}", alert_type);
    let body = format!(
        "An alert has been triggered in your Atmos system.\n\nAlert Type: {}\n\nDetails: {}",
        alert_type, details
    );
    (subject, body)
}
