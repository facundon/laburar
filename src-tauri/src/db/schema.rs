// @generated automatically by Diesel CLI.

diesel::table! {
    area (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    assignment (id) {
        id -> Integer,
        task_id -> Integer,
        area_id -> Integer,
        difficulty -> Integer,
        frequency -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

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
    employee_assignment (id) {
        id -> Integer,
        employee_id -> Integer,
        assignment_id -> Integer,
        is_primary -> Nullable<Bool>,
        efficiency -> Integer,
        created_at -> Nullable<Timestamp>,
        assigned_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(assignment -> area (area_id));
diesel::joinable!(assignment -> task (task_id));
diesel::joinable!(employee_assignment -> assignment (assignment_id));
diesel::joinable!(employee_assignment -> employee (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    area,
    assignment,
    employee,
    employee_assignment,
    task,
);
