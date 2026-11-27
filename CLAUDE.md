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
4. **Multiuser Security:** Content belongs to the company, not a user. Every query for
   content MUST (a) check the `AuthSession` user's employment covers now(), and (b) require the needed level from
   `effective_access` for that object type on its calendar or channel: `read` to view,
   `edit` to create, change or delete. Never take a user, calendar or channel
   id from the request as proof of access. Admin-only actions (calendars, channels, groups,
   roles, grants) need a current admin assignment. Reading the audit log needs admin,
   auditor, or `read` on `audit_log` for that calendar. Auditors never get `edit`.
   "Current" always means valid at now(). Store every date and time as TIMESTAMPTZ
   (UTC), never DATE, and show it in the viewer's own time zone.
   **No deletes:** Never write `DELETE` for app data. Hide things by setting `archived_at`
   (who did it is in the audit log); end people and their roles with an end date. Normal pages filter
   `archived_at IS NULL`; only the audit view shows archived rows.
5. **Plain CSS:** Rely on Pico.css semantic tags (`<main>`, `<article>`, `<form>`, `<input>`, `<button>`, `<table>`). Do not write custom CSS classes unless strictly necessary.
6. **Audit:** Every write runs in a transaction that first sets `app.user_id`, `app.ip` and
   `app.request_id`, so the audit triggers can record who did it. Auth events (login,
   failed login, logout, password change) and access-denied responses are logged by the
   app. Never update or delete `audit_events`.
7. **Security:** Every POST carries and checks a CSRF token. Never disable Askama escaping.
   New CDN scripts or styles need a pinned version and an integrity hash. Passwords,
   temporary ones included, are only stored as Argon2id hashes and never logged.
   There is no sign-up route; only admins create users.
8. **No impossible states:** The database must not be able to hold data that makes no
   sense. Enforce every rule in the schema first: structure (required links, separate
   tables instead of optional parents, composite foreign keys), then CHECK, UNIQUE and
   EXCLUDE constraints. Use a trigger only when none of those can express it. Rust code may
   add friendlier checks in front, but never replaces a database rule.
