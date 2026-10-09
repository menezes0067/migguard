# migguard

A linter that catches dangerous SQL migrations before they reach production.

> **Status:** early development. The project is a work in progress and the rule set is still small.

## Why it exists

Some migrations look harmless but can take production down. A `CREATE INDEX` without `CONCURRENTLY` blocks writes on the table while the index is built. An `ADD COLUMN ... NOT NULL` without a default fails on any table that already has rows. A `DROP COLUMN` can break the previous version of the application that is still running during a deploy.

These problems usually slip through because they work fine in development and staging, where tables are small and nothing is under load. They tend to be caught only in code review, when they are caught at all.

`migguard` reads your migration files, flags the risky statements, explains why each one is dangerous, and suggests a safer alternative. It is meant to run locally and in CI, failing the pipeline before the problem reaches production.

## Usage

Point `migguard` at a `.sql` file:

```bash
migguard migrations/002_add_index.sql
```

Given this migration:

```sql
CREATE TABLE orders (id bigint PRIMARY KEY, user_id bigint);

CREATE INDEX idx_orders_user_id ON orders (user_id);
```

`migguard` reports:

```
migrations/002_add_index.sql:3: error [create-index-concurrently] CREATE INDEX without CONCURRENTLY blocks writes on the table while the index is built
    help: use CREATE INDEX CONCURRENTLY

2 statement(s) analyzed: 1 error(s)
```

The safe version passes cleanly:

```sql
CREATE INDEX CONCURRENTLY idx_orders_user_id ON orders (user_id);
```

### Exit codes

| Code | Meaning |
|------|---------|
| `0`  | No errors found (warnings do not fail the run) |
| `1`  | At least one error was found, or the file could not be read |

This makes it easy to use in CI: a dangerous migration fails the pipeline.

### Running from source

```bash
cargo run -- path/to/migration.sql
```

Or install it so the `migguard` command is available everywhere:

```bash
cargo install --path .
```

## Design goals

- **Educational messages.** A finding should tell you what is wrong, why it matters, and what to do instead.
- **Few false positives.** A linter that cries wolf gets ignored, so uncertain cases are warnings rather than errors.
- **Easy to extend.** Each rule is a small, self-contained module, so adding a new one should take a single file.
- **Honest about its limits.** Known limitations are documented rather than hidden.

## Scope

Focused on PostgreSQL for now. Lock behavior differs between databases and between versions, so a rule that matters in one engine may be harmless in another.

