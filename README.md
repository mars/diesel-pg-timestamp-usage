# Timestamps with Diesel.rs & Postgres

Example using UTC timestamps with the Rust language database ORM [Diesel](https://diesel.rs).

🌱💚 *Please note, I am new to Rust. I could not find a clear example for this basic task, so here's what I figured out. No guarantees about correctness. Please [open an issue](https://github.com/mars/diesel-pg-timestamp-usage/issues) to give feedback. I'd love to find better ways to do this.* 

## Requirements

* [Rust + Cargo](https://www.rust-lang.org)
* [Postgres](https://www.postgresql.org/download/)
* `cargo install diesel_cli`

## Usage

```bash
export DATABASE_URL=postgres://postgres@localhost:5432/diesel-pg-timestamp-usage
diesel setup
cargo run
```

## Original setup

```bash
export DATABASE_URL=postgres://postgres@localhost:5432/diesel-pg-timestamp-usage
diesel setup
diesel migration generate create-example
# Write code in `migrations/up.sql` & `down.sql`
diesel migration run
diesel migration redo
diesel print-schema > src/schema.rs
```
