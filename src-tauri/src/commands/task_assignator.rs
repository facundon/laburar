use crate::db::models::assignment::{list_assignments_without_employees, AssignmentWithNames};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::modules::task_assignator::{suggest_employees_for_assignation, EmployeeWithScore};
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn suggest_employees_for_assignation_command(
    assignment_id: i32,
    assignation_start_date: String,
    assignation_end_date: String,
    current_employee_id: i32,
) -> Result<Vec<EmployeeWithScore>, Error> {
    let mut conn = establish_connection();
    suggest_employees_for_assignation(
        &mut conn,
        assignment_id,
        parse_date(&assignation_start_date).unwrap(),
        parse_date(&assignation_end_date).unwrap(),
        current_employee_id,
    )
}

#[command(rename_all = "snake_case")]
pub fn list_assignments_without_employees_command() -> Result<Vec<AssignmentWithNames>, Error> {
    let mut conn = establish_connection();
    list_assignments_without_employees(&mut conn)
}
