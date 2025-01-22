use crate::db::models::holiday::{
    create_holiday, delete_holiday, get_holiday, list_holidays, update_holiday, Holiday,
    HolidayWithEmployee,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_holiday_command(
    employee_id: i32,
    start_date: &str,
    end_date: &str,
    days_off: i32,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    let mut conn = establish_connection();

    create_holiday(
        &mut conn,
        employee_id,
        parse_date(start_date)?,
        parse_date(end_date)?,
        days_off,
        notes,
    )
}

#[command(rename_all = "snake_case")]
pub fn get_holiday_command(id: i32) -> Result<HolidayWithEmployee, Error> {
    let mut conn = establish_connection();
    get_holiday(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_holidays_command() -> Result<Vec<HolidayWithEmployee>, Error> {
    let mut conn = establish_connection();
    list_holidays(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_holiday_command(
    id: i32,
    employee_id: i32,
    start_date: &str,
    end_date: &str,
    days_off: i32,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    let mut conn = establish_connection();
    update_holiday(
        &mut conn,
        id,
        employee_id,
        parse_date(start_date)?,
        parse_date(end_date)?,
        days_off,
        notes,
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_holiday_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_holiday(&mut conn, id)
}
