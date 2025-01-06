pub use crate::db::models::employee::create_employee;
use crate::db::sqlite::establish_connection;
use chrono::NaiveDateTime;

#[tauri::command]
pub fn create_employee_command(
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<i32>,
    start_date: Option<NaiveDateTime>,
) -> String {
    let mut conn = establish_connection();
    let employee = create_employee(&mut conn, address, first_name, last_name, phone.as_ref(), start_date.as_ref());
    serde_json::to_string(&employee).unwrap()
}
