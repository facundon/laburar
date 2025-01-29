use crate::db::schema::absence_return;
use crate::error::Error;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Default, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = absence_return)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AbsenceReturn {
    pub id: i32,
    pub absence_id: i32,
    pub returned_hours: i32,
    pub notes: Option<String>,
    pub return_date: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = absence_return)]
pub struct NewAbsenceReturn<'a> {
    pub absence_id: i32,
    pub returned_hours: i32,
    pub notes: Option<&'a str>,
    pub return_date: &'a NaiveDate,
}

pub fn create_absence_return(
    conn: &mut SqliteConnection,
    absence_id: i32,
    returned_hours: i32,
    notes: Option<&str>,
    return_date: &NaiveDate,
) -> Result<AbsenceReturn, Error> {
    let new_absence_return = NewAbsenceReturn {
        absence_id,
        returned_hours,
        notes,
        return_date,
    };
    diesel::insert_into(absence_return::table)
        .values(&new_absence_return)
        .returning(AbsenceReturn::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn get_absence_return(conn: &mut SqliteConnection, id: i32) -> Result<AbsenceReturn, Error> {
    absence_return::table
        .find(id)
        .first(conn)
        .map_err(Error::Database)
}

pub fn list_absence_returns(conn: &mut SqliteConnection) -> Result<Vec<AbsenceReturn>, Error> {
    absence_return::table.load(conn).map_err(Error::Database)
}

pub fn update_absence_return(
    conn: &mut SqliteConnection,
    id: i32,
    absence_id: i32,
    returned_hours: i32,
    notes: Option<&str>,
    return_date: &NaiveDate,
) -> Result<AbsenceReturn, Error> {
    let updated_absence_return = NewAbsenceReturn {
        absence_id,
        returned_hours,
        notes,
        return_date,
    };
    diesel::update(absence_return::table.find(id))
        .set(&updated_absence_return)
        .returning(AbsenceReturn::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn delete_absence_return(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(absence_return::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}
