pub use crate::db::models::absence_return::{
    create_absence_return, delete_absence_return, get_absence_return, list_absence_returns,
    update_absence_return, AbsenceReturn,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_absence_return_command(
    absence_id: i32,
    returned_hours: i32,
    return_date: &str,
    notes: Option<&str>,
) -> Result<AbsenceReturn, Error> {
    let mut conn = establish_connection();
    let parsed_date = parse_date(return_date)?;
    create_absence_return(&mut conn, absence_id, returned_hours, notes, &parsed_date)
}

#[command(rename_all = "snake_case")]
pub fn get_absence_return_command(id: i32) -> Result<AbsenceReturn, Error> {
    let mut conn = establish_connection();
    get_absence_return(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_absence_returns_command() -> Result<Vec<AbsenceReturn>, Error> {
    let mut conn = establish_connection();
    list_absence_returns(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_absence_return_command(
    id: i32,
    absence_id: i32,
    returned_hours: i32,
    return_date: &str,
    notes: Option<&str>,
) -> Result<AbsenceReturn, Error> {
    let mut conn = establish_connection();
    let parsed_date = parse_date(return_date)?;
    update_absence_return(
        &mut conn,
        id,
        absence_id,
        returned_hours,
        notes,
        &parsed_date,
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_absence_return_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_absence_return(&mut conn, id)
}
