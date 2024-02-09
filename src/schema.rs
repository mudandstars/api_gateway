// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        key -> Varchar,
        user_id -> Unsigned<Integer>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    logs (id) {
        id -> Unsigned<Integer>,
        api_key_id -> Unsigned<Integer>,
        #[max_length = 6]
        method -> Varchar,
        #[max_length = 255]
        uri -> Varchar,
        status -> Unsigned<Smallint>,
        duration_in_microseconds -> Unsigned<Bigint>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        email -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    logs,
    users,
);
