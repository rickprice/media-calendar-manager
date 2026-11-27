# Project Rules & Tech Stack

## Stack
- **Language:** Rust (2021 edition)
- **Web Framework:** `axum` (0.8)
- **Database:** PostgreSQL via `sqlx` (0.9) — NO heavy ORMs. Use `sqlx::query!` or `sqlx::query_as!`.
- **Auth & Sessions:** `axum-login` (0.19) + `tower-sessions` (0.15) with `tower-sessions-sqlx-store` (Postgres) + `password-auth` (Argon2).
- **Templating:** `askama` (0.16) — Server-side rendered HTML only. Do NOT build a JSON REST API.
- **Frontend:** Plain semantic HTML5 + Pico.css (CDN) + HTMX (CDN) for partial swaps where helpful. Zero custom JavaScript files.

## Coding & Workflow Guardrails
1. **Fast Verification:** Always run `cargo check` while iterating on code. Only run `cargo build` or `cargo run` when testing the running server.
2. **Database Migrations:** All schema changes must go into `migrations/` as numbered `.sql` files (e.g., `0001_create_users.sql`). Run `sqlx migrate run` before compiling so `sqlx` compile-time macros verify against the live schema.
3. **Error Handling:** Use a single `AppError` enum in `src/error.rs` implementing `axum::response::IntoResponse`. Avoid `.unwrap()` or `.expect()` inside route handlers.
4. **Multiuser Security:** Every query for user-owned resources MUST filter by `user_id` extracted from the authenticated `AuthSession`.
5. **Plain CSS:** Rely on Pico.css semantic tags (`<main>`, `<article>`, `<form>`, `<input>`, `<button>`, `<table>`). Do not write custom CSS classes unless strictly necessary.
