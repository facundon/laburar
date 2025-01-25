use crate::db::models::replacement::{
    create_replacement, delete_replacement, get_replacement, list_replacements, update_replacement,
    Replacement, ReplacementWithEmployees,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use chrono::NaiveDate;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_replacement_command(
    original_employee_id: i32,
    replacement_employee_id: i32,
    replacement_start_date: NaiveDate,
    replacement_end_date: NaiveDate,
    assignment_id: i32,
    notes: Option<String>,
) -> Result<Replacement, Error> {
    let mut conn = establish_connection();
    create_replacement(
        &mut conn,
        original_employee_id,
        replacement_employee_id,
        replacement_start_date,
        replacement_end_date,
        assignment_id,
        notes.as_deref(),
    )
}

#[command(rename_all = "snake_case")]
pub fn get_replacement_command(id: i32) -> Result<ReplacementWithEmployees, Error> {
    let mut conn = establish_connection();
    get_replacement(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_replacements_command() -> Result<Vec<ReplacementWithEmployees>, Error> {
    let mut conn = establish_connection();
    list_replacements(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_replacement_command(
    id: i32,
    original_employee_id: i32,
    replacement_employee_id: i32,
    replacement_start_date: NaiveDate,
    replacement_end_date: NaiveDate,
    assignment_id: i32,
    notes: Option<String>,
) -> Result<Replacement, Error> {
    let mut conn = establish_connection();
    update_replacement(
        &mut conn,
        id,
        original_employee_id,
        replacement_employee_id,
        replacement_start_date,
        replacement_end_date,
        assignment_id,
        notes.as_deref(),
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_replacement_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_replacement(&mut conn, id)
}
