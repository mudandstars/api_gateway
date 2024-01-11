use diesel::prelude::*;
use api_gateway::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{target}%");

    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(users.filter(email.like(pattern)))
        .execute(connection)
        .expect("Error deleting users");

    println!("Deleted {num_deleted} users");
}