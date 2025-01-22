use crate::db::schema::holiday;
use crate::error::Error;
use chrono::NaiveDate;
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = holiday)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Holiday {
    pub id: i32,
    pub employee_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_off: i32,
    pub notes: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = holiday)]
pub struct NewHoliday<'a> {
    pub employee_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_off: i32,
    pub notes: Option<&'a str>,
}

pub fn create_holiday(
    conn: &mut SqliteConnection,
    employee_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    days_off: i32,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    let new_holiday = NewHoliday {
        employee_id,
        start_date,
        end_date,
        days_off,
        notes,
    };

    diesel::insert_into(holiday::table)
        .values(&new_holiday)
        .returning(Holiday::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_holiday(conn: &mut SqliteConnection, id: i32) -> Result<Holiday, Error> {
    holiday::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_holidays(conn: &mut SqliteConnection) -> Result<Vec<Holiday>, Error> {
    holiday::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_holiday(
    conn: &mut SqliteConnection,
    id: i32,
    employee_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    days_off: i32,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    diesel::update(holiday::table.find(id))
        .set((
            holiday::employee_id.eq(employee_id),
            holiday::start_date.eq(start_date),
            holiday::end_date.eq(end_date),
            holiday::days_off.eq(days_off),
            holiday::notes.eq(notes),
        ))
        .returning(Holiday::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_holiday(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(holiday::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
