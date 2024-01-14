#[macro_use] extern crate rocket;

#[get("/world")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world2")]
fn index2() -> &'static str {
    "Hello, world2!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![index, index2])
}
