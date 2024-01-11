// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        key -> Varchar,
        user_id -> Unsigned<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    users,
);
