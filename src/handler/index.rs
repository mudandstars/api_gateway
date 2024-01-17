use rocket;

#[rocket::get("/world")]
pub fn index() -> String {
    String::from("Hello, world!")
}

#[rocket::get("/world2")]
pub fn index2() -> String {
    String::from("Hello, world2!")
}
