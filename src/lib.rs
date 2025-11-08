pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Please ensure DB URL is set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn display_users(conn: &mut PgConnection) {
    use crate::schema::users;

    let results = users::table.select(User::as_select()).load(conn).unwrap();

    match results.len() {
        0 => println!("There are currently 0 users recorded"),
        _ => println!("Displaying {} users", results.len()),
    }

    for user in results {
        println!("Id: {}", user.id);
        println!("Name: {}", user.name);
        println!("Age: {}", user.age);
    }
}
