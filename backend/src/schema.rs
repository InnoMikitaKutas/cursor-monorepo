// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Uuid,
        user_id -> Uuid,
        street -> Varchar,
        suite -> Nullable<Varchar>,
        city -> Varchar,
        zipcode -> Varchar,
        lat -> Nullable<Numeric>,
        lng -> Nullable<Numeric>,
    }
}

diesel::table! {
    auth_users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    companies (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        catch_phrase -> Nullable<Varchar>,
        bs -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(addresses -> users (user_id));
diesel::joinable!(companies -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    auth_users,
    companies,
    users,
); 