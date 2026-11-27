use axum::{
    body::Body,
    http::{header, Request, Response, StatusCode},
};
use http_body_util::BodyExt;
use media_calendar_manager::build_router;
use sqlx::PgPool;
use tower::ServiceExt;

async fn body_string(response: Response<Body>) -> String {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn get(uri: &str, cookie: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method("GET").uri(uri);
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    builder.body(Body::empty()).unwrap()
}

fn post_form(uri: &str, form: &str, cookie: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder()
        .method("POST")
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    if let Some(cookie) = cookie {
        builder = builder.header(header::COOKIE, cookie);
    }
    builder.body(Body::from(form.to_string())).unwrap()
}

fn redirect_location(response: &Response<Body>) -> &str {
    response
        .headers()
        .get(header::LOCATION)
        .expect("expected a Location header")
        .to_str()
        .unwrap()
}

/// Extracts the `name=value` pair from a `Set-Cookie` header, discarding
/// attributes like `Path`/`HttpOnly`, so it can be replayed as a `Cookie`
/// header on the next request.
fn session_cookie(response: &Response<Body>) -> Option<String> {
    response
        .headers()
        .get(header::SET_COOKIE)?
        .to_str()
        .ok()
        .and_then(|value| value.split(';').next())
        .map(str::to_string)
}

#[sqlx::test]
async fn root_redirects_to_login(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    let response = app.oneshot(get("/", None)).await.unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/login");
}

#[sqlx::test]
async fn dashboard_redirects_to_login_when_unauthenticated(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    let response = app.oneshot(get("/dashboard", None)).await.unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert!(redirect_location(&response).starts_with("/login"));
}

#[sqlx::test]
async fn register_creates_user_and_redirects_to_login(pool: PgPool) {
    let app = build_router(pool.clone(), false).await.unwrap();

    let response = app
        .oneshot(post_form(
            "/register",
            "email=new%40example.com&password=correcthorsebatterystaple",
            None,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/login");

    let count: i64 = sqlx::query_scalar!(
        "SELECT count(*) FROM users WHERE email = $1",
        "new@example.com"
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .unwrap_or(0);
    assert_eq!(count, 1);
}

#[sqlx::test]
async fn register_duplicate_email_rerenders_form_with_error(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    let first = app
        .clone()
        .oneshot(post_form(
            "/register",
            "email=dup%40example.com&password=correcthorsebatterystaple",
            None,
        ))
        .await
        .unwrap();
    assert_eq!(first.status(), StatusCode::SEE_OTHER);

    let second = app
        .oneshot(post_form(
            "/register",
            "email=dup%40example.com&password=another-password",
            None,
        ))
        .await
        .unwrap();

    assert_eq!(second.status(), StatusCode::OK);
    assert!(body_string(second).await.contains("already exists"));
}

#[sqlx::test]
async fn login_with_wrong_password_rerenders_form_with_error(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    app.clone()
        .oneshot(post_form(
            "/register",
            "email=person%40example.com&password=correcthorsebatterystaple",
            None,
        ))
        .await
        .unwrap();

    let response = app
        .oneshot(post_form(
            "/login",
            "email=person%40example.com&password=wrong-password",
            None,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(body_string(response).await.contains("Invalid email or password"));
}

#[sqlx::test]
async fn login_with_unknown_email_rerenders_form_with_error(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    let response = app
        .oneshot(post_form(
            "/login",
            "email=nobody%40example.com&password=whatever",
            None,
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(body_string(response).await.contains("Invalid email or password"));
}

#[sqlx::test]
async fn logout_without_a_session_still_redirects_to_login(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/logout")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/login");
}

#[sqlx::test]
async fn full_register_login_dashboard_logout_flow(pool: PgPool) {
    let app = build_router(pool, false).await.unwrap();

    // Register.
    let response = app
        .clone()
        .oneshot(post_form(
            "/register",
            "email=flow%40example.com&password=correcthorsebatterystaple",
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/login");

    // Dashboard is still locked before logging in.
    let response = app.clone().oneshot(get("/dashboard", None)).await.unwrap();
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    // Log in.
    let response = app
        .clone()
        .oneshot(post_form(
            "/login",
            "email=flow%40example.com&password=correcthorsebatterystaple",
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/dashboard");
    let cookie = session_cookie(&response).expect("login should set a session cookie");

    // Dashboard now shows the logged-in user's email.
    let response = app
        .clone()
        .oneshot(get("/dashboard", Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = body_string(response).await;
    assert!(body.contains("flow@example.com"));

    // Log out.
    let response = app
        .clone()
        .oneshot(post_form("/logout", "", Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(redirect_location(&response), "/login");

    // Dashboard is locked again with the same (now logged-out) cookie.
    let response = app
        .oneshot(get("/dashboard", Some(&cookie)))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
}
