use api_gateway::*;
use std::io::{stdin};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut email = String::new();

    println!("What would you like your name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("What would you like your email to be?");
    stdin().read_line(&mut email).unwrap();
    let email = name.trim_end(); // Remove the trailing newline

    let user = create_user_with_api_key(connection, &name, &email);
    println!("\nSaved user '{}' with id:{}", name, user.id);
}
