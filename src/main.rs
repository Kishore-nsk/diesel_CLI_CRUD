use diesel_cli_crud::{
    create_user, delete_user, display_users, establish_connection, update_user_age,
};
use std::io::stdin;

fn main() {
    let mut user_input = String::new();

    println!("Hello, how would you like to interact with your users table?\n");
    println!("----------");
    println!("\nPress R if you would like to RETRIEVE the current list of users\n");
    println!("Press C if you would like to CREATE a new user\n");
    println!("Press U to update the age of an existing user\n");
    println!("Press D to delete an existing user\n");

    stdin()
        .read_line(&mut user_input)
        .expect("Please pass in a valid input");

    let connection = &mut establish_connection();

    match user_input.as_str().trim() {
        "R" => display_users(connection),
        "C" => {
            create_user(connection);
            println!("User added successfully");
        }
        "U" => {
            let user = update_user_age(connection);
            println!("{}'s age updated successfully to {}", user.name, user.age);
        }
        "D" => {
            let deleted_num = delete_user(connection);
            println!("{} user deleted successfully", deleted_num);
        }
        _ => println!("Please select from one of the valid options. (R/C/U)"),
    }
}
