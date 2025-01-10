pub use crate::db::models::employee::{
    assign_tasks_to_employee, create_employee, delete_employee, get_employee,
    get_employee_with_tasks, list_employees, update_employee,
};
use crate::db::models::task::Task;
use crate::db::{models::employee::Employee, sqlite::establish_connection};
use crate::error::Error;
use chrono::NaiveDateTime;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_employee_command(
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<String>,
) -> Result<Employee, Error> {
    let mut conn = establish_connection();

    let parsed_start_date: Option<NaiveDateTime> = match start_date {
        Some(date_str) => Some(
            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ")
                .map_err(|e| Error::DateParse(e))?,
        ),
        None => None,
    };

    create_employee(
        &mut conn,
        address,
        first_name,
        last_name,
        phone,
        parsed_start_date.as_ref(),
    )
}

#[command(rename_all = "snake_case")]
pub fn get_employee_command(id: i32) -> Result<Employee, Error> {
    let mut conn = establish_connection();
    get_employee(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_employees_command() -> Result<Vec<Employee>, Error> {
    let mut conn = establish_connection();
    list_employees(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_employee_command(
    id: i32,
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<String>,
) -> Result<Employee, Error> {
    let mut conn = establish_connection();

    let parsed_start_date: Option<NaiveDateTime> = match start_date {
        Some(date_str) => Some(
            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.fZ")
                .map_err(|e| Error::DateParse(e))?,
        ),
        None => None,
    };

    update_employee(
        &mut conn,
        id,
        address,
        first_name,
        last_name.trim(),
        phone,
        parsed_start_date.as_ref(),
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_employee_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_employee(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn get_employee_with_tasks_command(id: i32) -> Result<(Employee, Vec<Task>), Error> {
    let mut conn = establish_connection();
    get_employee_with_tasks(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn assign_tasks_to_employee_command(employee_id: i32, task_ids: Vec<i32>) -> Result<(), Error> {
    let mut conn = establish_connection();
    assign_tasks_to_employee(&mut conn, employee_id, task_ids)
}
