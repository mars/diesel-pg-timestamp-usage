use super::schema::examples;
use super::chrono;

#[derive(Insertable, Debug)]
#[table_name="examples"]
pub struct NewExample {
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct Example {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}