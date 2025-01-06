pub use crate::db::models::employee::create_employee;
use crate::db::{models::employee::Employee, sqlite::establish_connection};
use crate::error::Error;
use chrono::NaiveDateTime;

#[tauri::command(rename_all = "snake_case")]
pub fn create_employee_command(
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<String>,
) -> Result<Employee, Error> {
    let mut conn = establish_connection();

    let parsed_start_date: Option<NaiveDateTime> = match start_date {
        Some(date_str) => Some(NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ").map_err(|e| Error::DateParse(e))?),
        None => None,
    };

    create_employee(&mut conn, address, first_name, last_name, phone, parsed_start_date.as_ref())
}
