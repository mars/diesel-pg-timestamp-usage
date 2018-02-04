extern crate chrono;
#[macro_use]
extern crate diesel;

use chrono::prelude::{Utc};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

pub mod schema;
pub mod models;
use self::models::{Example, NewExample};

fn main() {
    let postgres_url = env::var("DATABASE_URL")
        .expect("requires DATABASE_URL env var");
    let pg = PgConnection::establish(&postgres_url)
        .expect(&format!("Error connecting to {}", postgres_url));

    let now = Utc::now().naive_utc();

    let new_example = NewExample {
        created_at: now,
        updated_at: Some(now),
    };

    let database_record = diesel::insert_into(schema::examples::table)
        .values(&new_example)
        .get_result::<Example>(&pg)
        .expect("Error saving new example");

    println!("Inserted example: {:?}", database_record);
}
