use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::modules::task_assignator::{sugest_employees_for_assignation, SuggestionResult};
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn sugest_employees_for_assignation_command(
    assignment_id: i32,
    start_date: String,
    end_date: String,
) -> Result<SuggestionResult, Error> {
    let mut conn = establish_connection();
    sugest_employees_for_assignation(
        &mut conn,
        assignment_id,
        parse_date(&start_date).unwrap(),
        parse_date(&end_date).unwrap(),
    )
}
