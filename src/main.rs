extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection; 
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url)
        .expect("Error connecting to database");


    let id_students= "12";
    let name = "John Doe";
    let new_student = models::Student { 
        id:id_students.to_string(),
        name_student:name.to_string(),
        created_at: chrono::Utc::now().naive_utc(), 
        }; 

    diesel::insert_into(schema::students::table)
        .values(&new_student)
        .execute(&mut conn)
        .expect("Error saving new student");
}
