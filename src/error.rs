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

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;

    use super::*;

    async fn body_string(response: Response) -> String {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    #[test]
    fn database_error_into_response_is_500() {
        let response = AppError::Database(sqlx::Error::RowNotFound).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn template_error_into_response_is_500() {
        let response = AppError::Template(askama::Error::Fmt).into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn internal_error_into_response_is_500() {
        let response = AppError::Internal.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn into_response_body_contains_the_display_message() {
        let response = AppError::Internal.into_response();
        assert_eq!(body_string(response).await, "internal server error");
    }

    #[test]
    fn axum_login_backend_error_converts_to_database_variant() {
        let err: AppError = axum_login::Error::<Backend>::Backend(sqlx::Error::RowNotFound).into();
        assert!(matches!(err, AppError::Database(_)));
    }

    #[test]
    fn sqlx_error_converts_via_from_into_database_variant() {
        let err: AppError = sqlx::Error::RowNotFound.into();
        assert!(matches!(err, AppError::Database(_)));
    }

    #[derive(Template)]
    #[template(source = "<p>hello {{ name }}</p>", ext = "html")]
    struct GreetingTemplate<'a> {
        name: &'a str,
    }

    #[tokio::test]
    async fn render_returns_ok_with_the_rendered_html() {
        let response = render(GreetingTemplate { name: "world" }).unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_string(response).await, "<p>hello world</p>");
    }
}
