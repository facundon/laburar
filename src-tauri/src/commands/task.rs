use crate::db::models::employee::Employee;
pub use crate::db::models::task::{
    create_task, delete_task, get_task, get_task_with_employees, get_tasks_for_employee,
    list_tasks, update_task,
};
use crate::db::{models::task::Task, sqlite::establish_connection};
use crate::error::Error;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_task_command(
    name: &str,
    description: Option<&str>,
    area: Option<&str>,
    difficulty: &str,
    frequency: &str,
) -> Result<Task, Error> {
    let mut conn = establish_connection();
    create_task(&mut conn, name, description, area, difficulty, frequency)
}

#[command(rename_all = "snake_case")]
pub fn get_task_command(id: i32) -> Result<Task, Error> {
    let mut conn = establish_connection();
    get_task(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_tasks_command() -> Result<Vec<Task>, Error> {
    let mut conn = establish_connection();
    list_tasks(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_task_command(
    id: i32,
    name: &str,
    description: Option<&str>,
    area: Option<&str>,
    difficulty: &str,
    frequency: &str,
) -> Result<Task, Error> {
    let mut conn = establish_connection();
    update_task(
        &mut conn,
        id,
        name,
        description,
        area,
        difficulty,
        frequency,
    )
}

#[command(rename_all = "snake_case")]
pub fn delete_task_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_task(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn get_task_with_employees_command(id: i32) -> Result<(Task, Vec<Employee>), Error> {
    let mut conn = establish_connection();
    get_task_with_employees(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn get_tasks_for_employee_command(employee_id: i32) -> Result<Vec<Task>, Error> {
    let mut conn = establish_connection();
    get_tasks_for_employee(&mut conn, employee_id)
}
