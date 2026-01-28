use std::sync::Arc;

use axum::extract::{State};
use axum::http::StatusCode;
use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get},
};

use crate::constant::constants::{API_EMAIL_SEND_PATH, API_EMAIL_MAIN_PATH};
use crate::dto::email::Email;
use crate::service::email_service::{DynEmailService, EmailService};

/// Email controller
pub trait EmailControllerTrait {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router;
}

/// Email controller implementation struct
pub struct EmailController {}

/// Email controller implementation logic
impl EmailControllerTrait for EmailController {
    /// Configure declared endpoints for this controller
    fn config_endpoints() -> Router {
        let email_service = Arc::new(EmailService::default()) as DynEmailService;
        Router::new()
            .nest(API_EMAIL_MAIN_PATH, create_routes())
            .with_state(email_service)
    }
}

/// Creates controller's routes
fn create_routes() -> Router<DynEmailService> {
    Router::new()
        .route(API_EMAIL_SEND_PATH, get(map_send_email))
}

/// Maps send email end-point
async fn map_send_email(
    State(email_service): State<DynEmailService>,
    Json(email): Json<Email>,
) -> impl IntoResponse {
    match email_service.send_emal(email).await {
        Ok(result) => Json(result).into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

/// Unit test cases
#[cfg(test)]
mod tests {
    // TODO - TO BE IMPLEMENTED
}
