use rocket;

#[rocket::get("/world")]
pub fn index() -> String {
    String::from("Hello, world!")
}

#[rocket::get("/world2")]
pub fn index2() -> String {
    String::from("Hello, world2!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error;

    #[test]
    fn test_can_store_a_user() {

       // make api test here
    }
}
