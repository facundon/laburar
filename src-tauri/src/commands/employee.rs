pub use crate::db::models::employee::{
    create_employee, delete_employee, get_employee, get_employee_with_assignments, list_employees,
    update_employee,
};
use crate::{
    db::{
        models::employee::{
            list_employees_on_holidays, list_employees_replacing_assignment, Employee,
            EmployeeOnHoliday, EmployeeWithAssignments,
        },
        sqlite::establish_connection,
    },
    error::Error,
    utils::parse_date_option,
};
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_employee_command(
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<&str>,
) -> Result<Employee, Error> {
    let mut conn = establish_connection();

    let parsed_start_date = parse_date_option(start_date)?;

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
pub fn get_employee_with_assignments_command(id: i32) -> Result<EmployeeWithAssignments, Error> {
    let mut conn = establish_connection();
    get_employee_with_assignments(&mut conn, id)
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
    start_date: Option<&str>,
) -> Result<Employee, Error> {
    let mut conn = establish_connection();

    let parsed_start_date = parse_date_option(start_date)?;

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
pub fn list_employees_on_holidays_command() -> Result<Vec<EmployeeOnHoliday>, Error> {
    let mut conn = establish_connection();
    list_employees_on_holidays(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn list_employees_replacing_assignment_command(assignment_id: i32) -> Result<Vec<i32>, Error> {
    let mut conn = establish_connection();
    list_employees_replacing_assignment(&mut conn, assignment_id)
}
