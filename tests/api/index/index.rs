use api_gateway::handler::index;
use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_index_route_returns_correct_content_type() {
    let rocket = rocket::build().mount("/", rocket::routes![index::index]);
    let client = Client::tracked(rocket).expect("valid rocket instance");

    let response = client.get("/world").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
}
