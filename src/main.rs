use api_gateway::handler;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/hello",
            routes![handler::index::index, handler::index::index2],
        )
        .mount("/users", routes![handler::users::store])
}
