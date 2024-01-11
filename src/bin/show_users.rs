use self::models::*;
use diesel::prelude::*;
use api_gateway::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("id: {}, name: {}, email: {}", user.id, user.name, user.email);
        println!("-----------\n");
    }
}
