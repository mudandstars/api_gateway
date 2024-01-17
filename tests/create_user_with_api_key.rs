use diesel::result::Error;
use diesel::prelude::*;
use api_gateway::{establish_connection, store_user_with_api_key};

#[test]
fn test_can_store_a_user() {
    establish_connection().test_transaction::<_, Error, _>(|conn| {
        let name = "test user";
        let email = "test@user.com";

        let user = store_user_with_api_key(conn, name, email);

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        assert_eq!(user.api_keys(conn).len(), 1);

        Ok(())
    });
}
