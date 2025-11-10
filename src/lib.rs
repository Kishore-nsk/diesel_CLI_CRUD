pub mod models;
pub mod schema;

use self::models::*;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::io::stdin;

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
        println!("Age: {}\n", user.age);
    }
}

pub fn create_user(conn: &mut PgConnection) -> User {
    let mut name: String = String::new();
    let mut age: String = String::new();

    println!("\nPlease type the name:");
    stdin()
        .read_line(&mut name)
        .expect("Please enter a valid name");

    println!("\nPlease type the age: ");
    stdin()
        .read_line(&mut age)
        .expect("Please enter a valid age");

    let new_age: i32 = age.trim().parse().unwrap();

    let new_user = NewUser {
        name: &name.trim(),
        age: new_age,
    };

    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error creating new user")
}

pub fn update_user_age(conn: &mut PgConnection) -> User {
    let mut read_id: String = String::new();
    let mut read_age = String::new();
    println!("Please enter the relevant User ID");

    stdin()
        .read_line(&mut read_id)
        .expect("Please enter a valid ID");

    let user_id: i32 = read_id.trim().parse().unwrap();

    println!("Please enter the new age: ");
    stdin()
        .read_line(&mut read_age)
        .expect("Please enter a valid age");

    let new_age: i32 = read_age.trim().parse().unwrap();

    use crate::schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(age.eq(new_age))
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error updating user age")
}

pub fn delete_user(conn: &mut PgConnection) -> usize {
    use crate::schema::users::dsl::*;

    let mut read_id = String::new();
    println!("Please enter the ID of the user you would like to delete");
    stdin()
        .read_line(&mut read_id)
        .expect("Please enter a valid ID number");

    let user_id: i32 = read_id.trim().parse().unwrap();

    let deleted_num = diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting user");

    deleted_num
}
