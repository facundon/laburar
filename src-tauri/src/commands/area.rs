pub use crate::db::models::area::{
    create_area, delete_area, get_area, get_area_with_assignments, list_areas,
    list_areas_without_tasks, update_area, Area, AreaWithAssignments,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_area_command(name: &str, description: Option<&str>) -> Result<Area, Error> {
    let mut conn = establish_connection();
    create_area(&mut conn, name, description)
}

#[command(rename_all = "snake_case")]
pub fn get_area_command(id: i32) -> Result<Area, Error> {
    let mut conn = establish_connection();
    get_area(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn get_area_with_assignments_command(id: i32) -> Result<AreaWithAssignments, Error> {
    let mut conn = establish_connection();
    get_area_with_assignments(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_areas_command() -> Result<Vec<Area>, Error> {
    let mut conn = establish_connection();
    list_areas(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn list_areas_without_tasks_command() -> Result<Vec<Area>, Error> {
    let mut conn = establish_connection();
    list_areas_without_tasks(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_area_command(id: i32, name: &str, description: Option<&str>) -> Result<Area, Error> {
    let mut conn = establish_connection();
    update_area(&mut conn, id, name, description)
}

#[command(rename_all = "snake_case")]
pub fn delete_area_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_area(&mut conn, id)
}
