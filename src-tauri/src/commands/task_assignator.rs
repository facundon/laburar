use crate::db::sqlite::establish_connection;
use crate::modules::task_assignator::sugest_employees_for_assignation;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn sugest_employees_for_assignation_command(assignment_id: i32) -> () {
    let mut conn = establish_connection();
    sugest_employees_for_assignation(&mut conn, assignment_id);
}
