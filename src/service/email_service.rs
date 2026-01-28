use std::sync::Arc;
use async_trait::async_trait;
use log::info;
use crate::dto::email::Email;

/// Email service
#[async_trait]
pub trait EmailServiceTrait {
    /// Sends email based on [Email] values
    async fn send_emal(&self, email: Email) -> Result<(), ()>;
}

/// Email service implementation struct
pub struct EmailService {
}

/// default initialization
impl Default for EmailService {
    fn default() -> Self {
        EmailService {}
    }
}

/// Email service implementation logic
#[async_trait]
impl EmailServiceTrait for EmailService {
    /// Sends email based on [Email] values
    async fn send_emal(&self, email: Email) -> Result<(), ()> {
        info!("send_emal - start");
        // TODO TO BE IMPLEMENTED
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
