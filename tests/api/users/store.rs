use api_gateway::database;
use api_gateway::handler::users as users_handler;
use api_gateway::models::{NewUser, User};
use api_gateway::schema::users::name;
use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json;

use diesel::prelude::*;

#[test]
fn test_user_can_be_stored() {
    use api_gateway::schema::users::dsl::users;

    let rocket = rocket::build().mount("/", rocket::routes![users_handler::store]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let new_user = NewUser {
        name: "example_user",
        email: "user@example.com",
    };
    let request_body = serde_json::to_string(&new_user).expect("Failed to serialize new_user");

    let response = client
        .post("/store")
        .body(request_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let user = users
        .filter(name.eq(new_user.name))
        .first::<User>(&mut database::establish_connection());

    match user {
        Ok(user) => assert_eq!(user.name, new_user.name),
        Err(_) => panic!("An error occurred while fetching user {}", new_user.name),
    }
}
