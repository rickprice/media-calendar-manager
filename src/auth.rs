use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use axum_login::{AuthnBackend, UserId};
use password_auth::{generate_hash, verify_password};
use sqlx::PgPool;

use crate::{
    error::{render, AppError},
    models::{Credentials, RegisterForm, User},
    AppState,
};

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone)]
pub struct Backend {
    pool: PgPool,
}

impl Backend {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, email, password_hash FROM users WHERE email = $1",
            creds.email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.filter(|user| verify_password(&creds.password, &user.password_hash).is_ok()))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        sqlx::query_as!(
            User,
            "SELECT id, email, password_hash FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    error: Option<String>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", get(register_form).post(register))
        .route("/login", get(login_form).post(login))
        .route("/logout", post(logout))
}

async fn register_form() -> Result<Response, AppError> {
    render(RegisterTemplate { error: None })
}

async fn register(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Result<Response, AppError> {
    let password_hash = generate_hash(&form.password);

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2)",
        form.email,
        password_hash
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Ok(Redirect::to("/login").into_response()),
        Err(err) if err.as_database_error().is_some_and(|e| e.is_unique_violation()) => {
            render(RegisterTemplate {
                error: Some("An account with that email already exists.".to_string()),
            })
        }
        Err(err) => Err(AppError::Database(err)),
    }
}

async fn login_form() -> Result<Response, AppError> {
    render(LoginTemplate { error: None })
}

async fn login(auth_session: AuthSession, Form(creds): Form<Credentials>) -> Result<Response, AppError> {
    let Some(user) = auth_session.authenticate(creds).await? else {
        return render(LoginTemplate {
            error: Some("Invalid email or password.".to_string()),
        });
    };

    auth_session.login(&user).await?;

    Ok(Redirect::to("/dashboard").into_response())
}

async fn logout(auth_session: AuthSession) -> Result<Response, AppError> {
    auth_session.logout().await?;
    Ok(Redirect::to("/login").into_response())
}
