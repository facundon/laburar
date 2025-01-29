pub use crate::db::models::employee_assignment::{
    create_assignments_to_employee, delete_employee_assignment, list_employee_assignments,
    update_employee_assignment,
};
use crate::db::models::replacement::delete_employee_replacements;
use crate::db::{
    models::employee_assignment::EmployeeAssignmentWithNames, sqlite::establish_connection,
};
use crate::error::Error;
use diesel::Connection;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_assignments_to_employee_command(
    employee_id: i32,
    assignment_ids: Vec<i32>,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    create_assignments_to_employee(&mut conn, employee_id, assignment_ids)
}

#[command(rename_all = "snake_case")]
pub fn list_employee_assignments_command(
    id: i32,
) -> Result<Vec<EmployeeAssignmentWithNames>, Error> {
    let mut conn = establish_connection();
    list_employee_assignments(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn update_employee_assignment_command(
    employee_id: i32,
    assignment_id: i32,
    efficiency: i32,
    is_primary: bool,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    update_employee_assignment(
        &mut conn,
        employee_id,
        assignment_id,
        efficiency,
        is_primary,
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_employee_assignment_command(
    employee_id: i32,
    assignment_id: i32,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    conn.transaction(|conn| {
        delete_employee_replacements(conn, employee_id)?;
        delete_employee_assignment(conn, employee_id, assignment_id)
    })
}
