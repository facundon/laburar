pub use crate::db::models::absence::{
    create_absence, delete_absence, get_absence, get_absence_with_returns, list_absences,
    update_absence, Absence, AbsenceWithReturns,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_absence_command(
    employee_id: i32,
    is_justified: bool,
    will_return: bool,
    hours: i32,
    description: Option<&str>,
    absence_type: &str,
    absence_date: &str,
) -> Result<Absence, Error> {
    let mut conn = establish_connection();
    let parsed_date = parse_date(absence_date)?;
    create_absence(
        &mut conn,
        employee_id,
        is_justified,
        will_return,
        hours,
        description,
        absence_type,
        parsed_date,
    )
}

#[command(rename_all = "snake_case")]
pub fn get_absence_command(id: i32) -> Result<Absence, Error> {
    let mut conn = establish_connection();
    get_absence(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn get_absence_with_returns_command(id: i32) -> Result<AbsenceWithReturns, Error> {
    let mut conn = establish_connection();
    get_absence_with_returns(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_absences_command() -> Result<Vec<Absence>, Error> {
    let mut conn = establish_connection();
    list_absences(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_absence_command(
    id: i32,
    employee_id: i32,
    is_justified: bool,
    will_return: bool,
    hours: i32,
    description: Option<&str>,
    absence_type: &str,
    absence_date: &str,
) -> Result<Absence, Error> {
    let mut conn = establish_connection();
    let parsed_date = parse_date(absence_date)?;
    update_absence(
        &mut conn,
        id,
        employee_id,
        is_justified,
        will_return,
        hours,
        description,
        absence_type,
        parsed_date,
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_absence_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_absence(&mut conn, id)
}
