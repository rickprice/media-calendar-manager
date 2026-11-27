use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

use crate::auth::Backend;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("template error: {0}")]
    Template(#[from] askama::Error),

    #[error("internal server error")]
    Internal,
}

impl From<axum_login::Error<Backend>> for AppError {
    fn from(err: axum_login::Error<Backend>) -> Self {
        match err {
            axum_login::Error::Backend(err) => AppError::Database(err),
            axum_login::Error::Session(_) => AppError::Internal,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;

        tracing::error!(error = %self, "request failed");

        (status, self.to_string()).into_response()
    }
}

/// Renders an Askama template into an HTML response, mapping render failures to `AppError`.
pub fn render<T: Template>(template: T) -> Result<Response, AppError> {
    Ok(Html(template.render()?).into_response())
}
