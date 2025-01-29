use crate::db::schema::company_holiday;
use crate::error::Error;
use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = company_holiday)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CompanyHoliday {
    pub id: i32,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = company_holiday)]
pub struct NewCompanyHoliday<'a> {
    pub date: NaiveDate,
    pub description: Option<&'a str>,
}

pub fn create_company_holiday(
    conn: &mut SqliteConnection,
    date: NaiveDate,
    description: Option<&str>,
) -> Result<CompanyHoliday, Error> {
    let new_company_holiday = NewCompanyHoliday { date, description };

    diesel::insert_into(company_holiday::table)
        .values(&new_company_holiday)
        .returning(CompanyHoliday::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn get_company_holiday(conn: &mut SqliteConnection, id: i32) -> Result<CompanyHoliday, Error> {
    company_holiday::table
        .find(id)
        .first(conn)
        .map_err(Error::Database)
}

pub fn list_company_holidays(conn: &mut SqliteConnection) -> Result<Vec<CompanyHoliday>, Error> {
    company_holiday::table
        .filter(company_holiday::date.gt(Local::now().date_naive()))
        .load(conn)
        .map_err(Error::Database)
}

pub fn update_company_holiday(
    conn: &mut SqliteConnection,
    id: i32,
    date: NaiveDate,
    description: Option<&str>,
) -> Result<CompanyHoliday, Error> {
    diesel::update(company_holiday::table.find(id))
        .set((
            company_holiday::date.eq(date),
            company_holiday::description.eq(description),
        ))
        .returning(CompanyHoliday::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn delete_company_holiday(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(company_holiday::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}
