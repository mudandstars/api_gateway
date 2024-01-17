use api_gateway::models::NewUser;
use api_gateway::store_user_with_api_key;
use api_gateway::database;
use diesel::result::Error;

use diesel::prelude::*;

#[test]
fn test_can_store_a_user() {
    database::establish_connection().test_transaction::<_, Error, _>(|conn| {
        let name = "test user";
        let email = "test@user.com";

        let user = store_user_with_api_key(conn, &NewUser { name, email });

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        assert_eq!(user.api_keys(conn).len(), 1);

        Ok(())
    });
}
