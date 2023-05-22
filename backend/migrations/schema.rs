// @generated automatically by Diesel CLI.

diesel::table! {
    Post (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
    }
}

diesel::table! {
    User (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
    }
}

diesel::table! {
    _prisma_migrations (id) {
        id -> Varchar,
        checksum -> Varchar,
        finished_at -> Nullable<Timestamptz>,
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Timestamptz>,
        started_at -> Timestamptz,
        applied_steps_count -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    Post,
    User,
    _prisma_migrations,
);
