// @generated automatically by Diesel CLI.

diesel::table! {
    absence (id) {
        id -> Integer,
        employee_id -> Integer,
        is_justified -> Bool,
        will_return -> Bool,
        hours -> Integer,
        description -> Nullable<Text>,
        absence_type -> Text,
        absence_date -> Date,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    absence_return (id) {
        id -> Integer,
        absence_id -> Integer,
        returned_hours -> Integer,
        notes -> Nullable<Text>,
        return_date -> Date,
        created_at -> Nullable<Timestamp>,
    }
}

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
        start_date -> Nullable<Date>,
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

diesel::joinable!(absence -> employee (employee_id));
diesel::joinable!(absence_return -> absence (absence_id));
diesel::joinable!(assignment -> area (area_id));
diesel::joinable!(assignment -> task (task_id));
diesel::joinable!(employee_assignment -> assignment (assignment_id));
diesel::joinable!(employee_assignment -> employee (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    absence,
    absence_return,
    area,
    assignment,
    employee,
    employee_assignment,
    task,
);
