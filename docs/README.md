# Design docs

- [data-model.md](data-model.md): glossary, data model, draft Postgres schema, access and audit rules, user-guide notes and open questions.

These are Markdown exports of the design docs we edit together in Claude Docs.
The draft SQL in `data-model.md` is a design, not a migration; real schema changes still go in `migrations/`.
When the design doc changes, export it again and replace the file here in the same pull request as any code that depends on it.
