#[cfg(test)]
mod tests {
    use api_gateway::{establish_connection, store_user_with_api_key};

    #[test]
    fn test_can_store_a_user() {
        let name = "test user";
        let email = "test@user.com";

        let user = store_user_with_api_key(&mut establish_connection(), name, email);

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        assert_eq!(user.api_keys(&mut establish_connection()).len(), 1);
    }
}
