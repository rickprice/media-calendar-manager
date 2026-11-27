pub mod auth;
pub mod error;
pub mod models;

use askama::Template;
use axum::{response::Redirect, routing::get, Router};
use axum_login::{login_required, AuthManagerLayerBuilder};
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{
    auth::{AuthSession, Backend},
    error::{render, AppError},
};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    email: String,
}

/// Builds the full application router: session middleware, auth middleware,
/// and all routes. Split out from `main` so tests can exercise the router
/// directly against a `sqlx::test`-provisioned pool.
///
/// `secure_cookies` should be `true` whenever the app is served over HTTPS
/// (production) and `false` over plain HTTP (local dev), since a `Secure`
/// session cookie is dropped by the browser on a non-HTTPS connection.
pub async fn build_router(pool: sqlx::PgPool, secure_cookies: bool) -> Result<Router, sqlx::Error> {
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let session_layer = SessionManagerLayer::new(session_store).with_secure(secure_cookies);

    let backend = Backend::new(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let state = AppState { pool };

    let protected: Router<AppState> = Router::new()
        .route("/dashboard", get(dashboard))
        .route_layer(login_required!(Backend, login_url = "/login"));

    let app = Router::new()
        .merge(protected)
        .merge(auth::router())
        .route("/", get(root))
        .with_state(state)
        .layer(auth_layer);

    Ok(app)
}

async fn root() -> Redirect {
    Redirect::to("/login")
}

async fn dashboard(auth_session: AuthSession) -> Result<axum::response::Response, AppError> {
    let user = auth_session.user().await.ok_or(AppError::Internal)?;
    render(DashboardTemplate { email: user.email })
}
