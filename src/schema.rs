// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    schema_migrations (version) {
        #[max_length = 128]
        version -> Varchar,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        session_verifier -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        otp_code_encrypted -> Varchar,
        otp_code_attempts -> Int4,
        otp_code_confirmed -> Bool,
        otp_code_sent -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        hashed_password -> Varchar,
        reset_password_selector -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamp>,
        reset_password_validator_hash -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    schema_migrations,
    sessions,
    users,
);
