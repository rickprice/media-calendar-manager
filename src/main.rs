use std::net::SocketAddr;

use axum_server::tls_rustls::RustlsConfig;
use media_calendar_manager::build_router;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let tls_paths = std::env::var("TLS_CERT_PATH")
        .ok()
        .zip(std::env::var("TLS_KEY_PATH").ok());

    let app = build_router(pool, tls_paths.is_some()).await?;

    match tls_paths {
        Some((cert_path, key_path)) => {
            let config = RustlsConfig::from_pem_file(cert_path, key_path).await?;
            let addr = SocketAddr::from(([0, 0, 0, 0], 443));
            tracing::info!("listening on {addr} (TLS)");
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await?;
        }
        None => {
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
            tracing::info!("listening on {}", listener.local_addr()?);
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
