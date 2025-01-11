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

diesel::table! {
    employee_on_task (id) {
        id -> Integer,
        employee_id -> Integer,
        task_id -> Integer,
        isPrimary -> Nullable<Bool>,
        efficiency -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        area -> Nullable<Text>,
        difficulty -> Integer,
        frequency -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(employee_on_task -> employee (employee_id));
diesel::joinable!(employee_on_task -> task (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    employee,
    employee_on_task,
    task,
);
