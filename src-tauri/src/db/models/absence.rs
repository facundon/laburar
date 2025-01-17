use crate::db::models::absence_return::AbsenceReturn;
use crate::db::schema::{absence, absence_return};
use crate::error::Error;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{prelude::*, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Default, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = absence)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Absence {
    pub id: i32,
    pub employee_id: i32,
    pub is_justified: bool,
    pub will_return: bool,
    pub hours: i32,
    pub description: Option<String>,
    pub absence_type: String,
    pub absence_date: NaiveDate,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbsenceWithReturns {
    #[serde(flatten)]
    pub absence: Absence,
    pub returns: Vec<AbsenceReturn>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = absence)]
pub struct NewAbsence<'a> {
    pub employee_id: i32,
    pub is_justified: bool,
    pub will_return: bool,
    pub hours: i32,
    pub description: Option<&'a str>,
    pub absence_type: &'a str,
    pub absence_date: NaiveDate,
}

pub fn create_absence(
    conn: &mut SqliteConnection,
    employee_id: i32,
    is_justified: bool,
    will_return: bool,
    hours: i32,
    description: Option<&str>,
    absence_type: &str,
    absence_date: NaiveDate,
) -> Result<Absence, Error> {
    let new_absence = NewAbsence {
        employee_id,
        is_justified,
        will_return,
        hours,
        description,
        absence_type,
        absence_date,
    };
    diesel::insert_into(absence::table)
        .values(&new_absence)
        .returning(Absence::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_absence(conn: &mut SqliteConnection, id: i32) -> Result<Absence, Error> {
    absence::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_absence_with_returns(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<AbsenceWithReturns, Error> {
    absence::table
        .left_join(absence_return::table.on(absence::id.eq(absence_return::absence_id)))
        .filter(absence::id.eq(id))
        .select((absence::all_columns, absence_return::all_columns.nullable()))
        .load::<(Absence, Option<AbsenceReturn>)>(conn)
        .map(|results| {
            let mut absence_with_returns = AbsenceWithReturns {
                absence: results[0].0.clone(),
                returns: Vec::new(),
            };
            for (_, absence_return) in results {
                if let Some(absence_return) = absence_return {
                    absence_with_returns.returns.push(absence_return);
                }
            }
            absence_with_returns
        })
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_absences(conn: &mut SqliteConnection) -> Result<Vec<Absence>, Error> {
    absence::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_absence(
    conn: &mut SqliteConnection,
    id: i32,
    employee_id: i32,
    is_justified: bool,
    will_return: bool,
    hours: i32,
    description: Option<&str>,
    absence_type: &str,
    absence_date: NaiveDate,
) -> Result<Absence, Error> {
    let updated_absence = NewAbsence {
        employee_id,
        is_justified,
        will_return,
        hours,
        description,
        absence_type,
        absence_date,
    };
    diesel::update(absence::table.find(id))
        .set(&updated_absence)
        .returning(Absence::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_absence(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(absence::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
