use crate::constant::constants::{
    APP_EMAIL_ADDRESS_DEFAULT, APP_EMAIL_ADDRESS_ENV_VAR, APP_EMAIL_PASSWORD_DEFAULT,
    APP_EMAIL_PASSWORD_ENV_VAR, APP_EMAIL_SMTP_SERVER_DEFAULT, APP_EMAIL_SMTP_SERVER_ENV_VAR,
};
use crate::dto::email::Email;
use async_trait::async_trait;
use lettre::message::SinglePart;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};
use std::sync::Arc;

/// Email service
#[async_trait]
pub trait EmailServiceTrait {
    /// Sends email based on [Email] values
    async fn send_emal(&self, email: Email) -> Result<(), ()>;
}

/// Email service implementation struct
pub struct EmailService {
    smtp_server: String,
    email_from: String,
    email_pwd: String,
}

/// default initialization
impl Default for EmailService {
    fn default() -> Self {
        EmailService {
            smtp_server: std::env::var(APP_EMAIL_SMTP_SERVER_ENV_VAR)
                .unwrap_or(String::from(APP_EMAIL_SMTP_SERVER_DEFAULT)),
            email_from: std::env::var(APP_EMAIL_ADDRESS_ENV_VAR)
                .unwrap_or(String::from(APP_EMAIL_ADDRESS_DEFAULT)),
            email_pwd: std::env::var(APP_EMAIL_PASSWORD_ENV_VAR)
                .unwrap_or(String::from(APP_EMAIL_PASSWORD_DEFAULT)),
        }
    }
}

/// Email service implementation logic
#[async_trait]
impl EmailServiceTrait for EmailService {
    /// Sends email based on [Email] values
    async fn send_emal(&self, email: Email) -> Result<(), ()> {
        info!("send_emal - start");

        // validate mandatory values
        if email.target_address.trim().is_empty() || email.email_body.trim().is_empty() {
            error!("send_emal - mandatory values are empty or not valid - email input: {email:?}");
            return Err(());
        }

        // creates email credentials and smtp transport sender
        let smtp_transport = SmtpTransport::starttls_relay(self.smtp_server.as_str())
            .unwrap()
            .credentials(Credentials::new(
                self.email_from.clone(),
                self.email_pwd.clone(),
            ))
            .build();

        // creates email initial configuration including from, reply to, subject and headers values
        let email_configuration = Message::builder()
            .from(self.email_from.clone().parse().unwrap())
            .reply_to(self.email_from.clone().parse().unwrap())
            .to(email.target_address.clone().parse().unwrap())
            .subject(
                email
                    .email_subject
                    .clone()
                    .unwrap_or(String::from("no-subject")),
            );

        // sample without any attachment, cc or bcc
        let email_message = email_configuration
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(email.email_body.clone().into_bytes()),
            )
            .unwrap();

        // sends the email
        match smtp_transport.send(&email_message) {
            Ok(_) => {
                info!("send_emal - completed");
                info!("send_emal - attributes: {email:?}");
            }
            Err(send_error) => {
                error!("send_emal - error: {send_error}");
                error!("send_emal - attributes: {email:?}");
            }
        }

        info!("send_emal - done");

        Ok(())
    }
}

/// Email service trait for API router state (based on Rust samples for Axum DI)
pub type DynEmailService = Arc<dyn EmailServiceTrait + Send + Sync>;

/// Unit test cases
#[cfg(test)]
mod tests {
    // TODO - TO BE IMPLEMENTED
}
