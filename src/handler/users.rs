use rocket;

#[rocket::post("/store")]
pub fn store() -> String {
    String::from("Hello, world!")
}
