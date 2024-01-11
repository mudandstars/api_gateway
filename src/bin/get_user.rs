use self::models::User;
use diesel::prelude::*;
use api_gateway::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::users;

    let user_id = args()
        .nth(1)
        .expect("get_post requires a user id")
        .parse::<u32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let user =users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<User>, otherwise it will throw an error

    match user {
        Ok(Some(user)) => println!("User with id: {} has a name: {}", user.id, user.name),
        Ok(None) => println!("Unable to find user {}", user_id),
        Err(_) => println!("An error occurred while fetching user {}", user_id),
    }
}
