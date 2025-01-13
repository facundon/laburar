pub use crate::db::models::area::{create_area, delete_area, get_area, list_areas, update_area};
use crate::db::{models::area::Area, sqlite::establish_connection};
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
pub fn list_areas_command() -> Result<Vec<Area>, Error> {
    let mut conn = establish_connection();
    list_areas(&mut conn)
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
