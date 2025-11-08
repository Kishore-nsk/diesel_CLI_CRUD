use diesel_cli_crud::{display_users, establish_connection};
use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    println!("Hello, how would you like to interact with your users table?\n");
    println!("----------");
    println!("\nPress R if you would like to RETRIEVE the current list of users.\n");
    println!("Press C if you would like to CREATE a new user.");

    stdin()
        .read_line(&mut user_input)
        .expect("Please pass in a valid input");

    let connection = &mut establish_connection();

    match user_input.as_str().trim() {
        "R" => display_users(connection),
        _ => println!("WIP"),
    }
}
