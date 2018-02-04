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
```

Now, build & run the program:

```
cargo run
```

🏁 Each time it's run, a new example record will be inserted with the current timestamp. You should see output like:

```
Inserted example: Example { id: 3, created_at: 2018-02-04T19:35:20.404570, updated_at: Some(2018-02-04T19:35:20.404570) }
```


## Architecture

As the [Diesel Getting Started guide](http://diesel.rs/guides/getting-started/) recommends:

* [src/migrations/](src/migrations/)
  * SQL commands to setup the database
  * generated by `diesel migration generate $NAME`
  * executed by `diesel migration run`
* [src/schema.rs](src/schema.rs)
  * the typed database schema
  * generated by `diesel print-schema`
* [src/models.rs](src/models.rs)
  * structs that represent Diesel records in Rust
  * created by hand

And for this example,

* [src/main.rs](src/main.rs)
  * the high-level database connection & query program


### Original setup

```bash
export DATABASE_URL=postgres://postgres@localhost:5432/diesel-pg-timestamp-usage
diesel setup
diesel migration generate create-example
# Write code in `migrations/up.sql` & `down.sql`
diesel migration run
diesel migration redo
diesel print-schema > src/schema.rs
# Write code in `src/models.rs`
# Write code in `src/main.rs`
```
