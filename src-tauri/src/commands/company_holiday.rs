use crate::db::models::company_holiday::{
    create_company_holiday, delete_company_holiday, get_company_holiday, list_company_holidays,
    update_company_holiday, CompanyHoliday,
};
use crate::db::sqlite::establish_connection;
use crate::error::Error;
use crate::utils::parse_date;
use tauri::command;

#[command(rename_all = "snake_case")]
pub fn create_company_holiday_command(
    date: &str,
    description: Option<&str>,
) -> Result<CompanyHoliday, Error> {
    let mut conn = establish_connection();

    create_company_holiday(&mut conn, parse_date(date)?, description)
}

#[command(rename_all = "snake_case")]
pub fn get_company_holiday_command(id: i32) -> Result<CompanyHoliday, Error> {
    let mut conn = establish_connection();
    get_company_holiday(&mut conn, id)
}

#[command(rename_all = "snake_case")]
pub fn list_company_holidays_command() -> Result<Vec<CompanyHoliday>, Error> {
    let mut conn = establish_connection();
    list_company_holidays(&mut conn)
}

#[command(rename_all = "snake_case")]
pub fn update_company_holiday_command(
    id: i32,
    date: &str,
    description: Option<&str>,
) -> Result<CompanyHoliday, Error> {
    let mut conn = establish_connection();
    update_company_holiday(&mut conn, id, parse_date(date)?, description)
}

#[command(rename_all = "snake_case")]
pub fn delete_company_holiday_command(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    delete_company_holiday(&mut conn, id)
}
