pub use crate::db::models::assignment::{
    create_assignment, delete_assignment, get_assignment, list_assignments, update_assignment,
};
use crate::db::{models::assignment::Assignment, sqlite::establish_connection};
use crate::error::Error;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_assignment_command(
    task_id: i32,
    area_id: i32,
    difficulty: i32,
    frequency: &str,
) -> Result<Assignment, Error> {
    let mut conn = establish_connection();
    create_assignment(&mut conn, task_id, area_id, difficulty, frequency)
}

#[command(rename_all = "snake_case")]
pub fn get_assignment_command(id: i32) -> Result<Assignment, Error> {
    let mut conn = establish_connection();
    get_assignment(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_assignments_command() -> Result<Vec<Assignment>, Error> {
    let mut conn = establish_connection();
    list_assignments(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_assignment_command(
    id: i32,
    difficulty: i32,
    frequency: &str,
) -> Result<Assignment, Error> {
    let mut conn = establish_connection();
    update_assignment(&mut conn, id, difficulty, frequency)
}

#[command(rename_all = "snake_case")]
pub fn delete_assignment_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_assignment(&mut conn, id)
}
