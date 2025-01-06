// @generated automatically by Diesel CLI.

diesel::table! {
    employee (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        phone -> Nullable<Text>,
        address -> Text,
        start_date -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
