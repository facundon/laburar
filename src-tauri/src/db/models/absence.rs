use std::collections::HashMap;

use crate::db::models::absence_return::AbsenceReturn;
use crate::db::schema::{absence, absence_return, employee};
use crate::error::Error;
use crate::utils::{parse_date, parse_date_time};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::sql_types::{Bool, Integer, Text};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbsenceWithEmployee {
    #[serde(flatten)]
    pub absence: Absence,
    pub is_returned: bool,
    pub employee_name: String,
}
#[derive(QueryableByName, Debug)]
struct AbsenceWithComputed {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type= Integer)]
    employee_id: i32,
    #[diesel(sql_type= Bool)]
    is_justified: bool,
    #[diesel(sql_type= Bool)]
    will_return: bool,
    #[diesel(sql_type= Integer)]
    hours: i32,
    #[diesel(sql_type= Text)]
    description: String,
    #[diesel(sql_type= Text)]
    absence_type: String,
    #[diesel(sql_type= Text)]
    absence_date: String,
    #[diesel(sql_type= Text)]
    created_at: String,
    #[diesel(sql_type= Bool)]
    is_returned: bool,
    #[diesel(sql_type= Text)]
    pub first_name: String,
    #[diesel(sql_type= Text)]
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbsenceWithReturns {
    #[serde(flatten)]
    pub absence: Absence,
    pub is_returned: bool,
    pub employee_name: String,
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
        .map_err(Error::Database)
}

pub fn get_absence(conn: &mut SqliteConnection, id: i32) -> Result<Absence, Error> {
    absence::table.find(id).first(conn).map_err(Error::Database)
}

pub fn get_absence_with_returns(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<AbsenceWithReturns, Error> {
    absence::table
        .left_join(absence_return::table)
        .left_join(employee::table)
        .filter(absence::id.eq(id))
        .select((
            absence::all_columns,
            absence_return::all_columns.nullable(),
            employee::first_name.nullable(),
            employee::last_name.nullable(),
        ))
        .load::<(
            Absence,
            Option<AbsenceReturn>,
            Option<String>,
            Option<String>,
        )>(conn)
        .map(|results| {
            let mut absence_with_returns = AbsenceWithReturns {
                absence: results[0].0.clone(),
                employee_name: format!(
                    "{} {}",
                    results[0].2.as_ref().unwrap(),
                    results[0].3.as_ref().unwrap()
                ),
                is_returned: false,
                returns: Vec::new(),
            };
            for (_, absence_return, _, _) in results {
                if let Some(absence_return) = absence_return {
                    absence_with_returns.returns.push(absence_return);
                }
            }
            absence_with_returns.returns.sort_by_key(|r| r.return_date);
            let total_returned_hours: i32 = absence_with_returns
                .returns
                .iter()
                .map(|r| r.returned_hours)
                .sum();
            if total_returned_hours >= absence_with_returns.absence.hours {
                absence_with_returns.is_returned = true;
            }
            absence_with_returns
        })
        .map_err(Error::Database)
}

pub fn list_absences(conn: &mut SqliteConnection) -> Result<Vec<AbsenceWithEmployee>, Error> {
    let query = r#"
        SELECT
            absence.id,
            absence.employee_id,
            absence.is_justified,
            absence.will_return,
            absence.hours,
            absence.description,
            absence.absence_type,
            absence.absence_date,
            absence.created_at,
            employee.first_name,
            employee.last_name,
            CASE
                WHEN absence.hours - IFNULL(SUM(absence_return.returned_hours), 0) <= 0 THEN TRUE
                ELSE FALSE
            END AS is_returned
        FROM absence
        INNER JOIN employee ON absence.employee_id = employee.id
        LEFT JOIN absence_return ON absence.id = absence_return.absence_id
        GROUP BY absence.id, absence.employee_id, absence.is_justified, absence.will_return, 
                 absence.hours, absence.description, absence.absence_type, absence.absence_date, 
                 absence.created_at, employee.first_name, employee.last_name
        ORDER BY absence.absence_date DESC
    "#;

    diesel::sql_query(query)
        .load::<AbsenceWithComputed>(conn)
        .and_then(|result| {
            result
                .into_iter()
                .map(|absence| {
                    let employee_name = format!("{} {}", absence.first_name, absence.last_name);
                    Ok(AbsenceWithEmployee {
                        employee_name,
                        is_returned: absence.is_returned,
                        absence: Absence {
                            absence_date: parse_date(&absence.absence_date).unwrap(),
                            created_at: Some(parse_date_time(&absence.created_at).unwrap()),
                            absence_type: absence.absence_type,
                            description: Some(absence.description),
                            employee_id: absence.employee_id,
                            hours: absence.hours,
                            id: absence.id,
                            is_justified: absence.is_justified,
                            will_return: absence.will_return,
                        },
                    })
                })
                .collect()
        })
        .map_err(Error::Database)
}

pub fn list_absences_for_employee(
    conn: &mut SqliteConnection,
    employee_id: i32,
) -> Result<Vec<AbsenceWithReturns>, Error> {
    absence::table
        .left_join(absence_return::table)
        .left_join(employee::table)
        .filter(absence::employee_id.eq(employee_id))
        .order_by(absence::absence_date.desc())
        .select((
            absence::all_columns,
            absence_return::all_columns.nullable(),
            employee::first_name.nullable(),
            employee::last_name.nullable(),
        ))
        .load::<(
            Absence,
            Option<AbsenceReturn>,
            Option<String>,
            Option<String>,
        )>(conn)
        .map(|results| {
            let mut absences_with_returns: HashMap<i32, AbsenceWithReturns> = HashMap::new();

            for (absence, absence_return, first_name, last_name) in results {
                let entry = absences_with_returns
                    .entry(absence.id)
                    .or_insert(AbsenceWithReturns {
                        absence,
                        employee_name: format!("{} {}", first_name.unwrap(), last_name.unwrap()),
                        is_returned: false,
                        returns: Vec::new(),
                    });

                if let Some(absence_return) = absence_return {
                    entry.returns.push(absence_return);
                }
            }

            absences_with_returns
                .into_iter()
                .map(|(_, mut v)| {
                    let total_returned_hours: i32 =
                        v.returns.iter().map(|r| r.returned_hours).sum();
                    if total_returned_hours >= v.absence.hours {
                        v.is_returned = true;
                    }
                    v
                })
                .collect()
        })
        .map_err(Error::Database)
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
        .map_err(Error::Database)
}

pub fn delete_absence(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(absence::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}
