# media-calendar-manager

A server-rendered authentication vertical slice built on `axum`, `askama`,
and `sqlx` — no JSON API, no custom JavaScript.

## Stack

- **axum** 0.8 — web framework
- **sqlx** 0.9 (Postgres) — compile-time checked queries
- **axum-login** 0.19 + **tower-sessions** 0.15 (`tower-sessions-sqlx-store`
  for Postgres-backed sessions) — auth & session management
- **password-auth** (Argon2) — password hashing
- **askama** 0.16 — server-side templates
- Pico.css + HTMX via CDN, zero custom JS

## Setup

1. Create a `.env` file with a `DATABASE_URL`:

   ```
   DATABASE_URL=postgres://user:password@localhost:5432/media-calendar-manager-app
   ```

2. Run migrations (requires [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)):

   ```
   sqlx migrate run
   ```

3. Check and run:

   ```
   cargo check
   cargo run
   ```

The server listens on `0.0.0.0:3000`.

## Routes

| Route | Method | Description |
|---|---|---|
| `/` | GET | Redirects to `/login` |
| `/register` | GET, POST | Create an account |
| `/login` | GET, POST | Log in |
| `/logout` | POST | End the session |
| `/dashboard` | GET | Shows the logged-in user's email (requires login) |

## License

BSD-3-Clause — see [LICENSE](LICENSE).
