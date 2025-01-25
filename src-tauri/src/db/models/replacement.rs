use crate::db::schema::{employee, replacement};
use crate::error::Error;
use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = replacement)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Replacement {
    pub id: i32,
    pub original_employee_id: i32,
    pub replacement_employee_id: i32,
    pub replacement_start_date: NaiveDate,
    pub replacement_end_date: NaiveDate,
    pub assignment_id: i32,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplacementWithEmployees {
    #[serde(flatten)]
    pub replacement: Replacement,
    pub original_employee_name: String,
    pub replacement_employee_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = replacement)]
pub struct NewReplacement<'a> {
    pub original_employee_id: i32,
    pub replacement_employee_id: i32,
    pub replacement_start_date: NaiveDate,
    pub replacement_end_date: NaiveDate,
    pub assignment_id: i32,
    pub notes: Option<&'a str>,
}

pub fn create_replacement(
    conn: &mut SqliteConnection,
    original_employee_id: i32,
    replacement_employee_id: i32,
    replacement_start_date: NaiveDate,
    replacement_end_date: NaiveDate,
    assignment_id: i32,
    notes: Option<&str>,
) -> Result<Replacement, Error> {
    let new_replacement = NewReplacement {
        original_employee_id,
        replacement_employee_id,
        replacement_start_date,
        replacement_end_date,
        assignment_id,
        notes,
    };

    diesel::insert_into(replacement::table)
        .values(&new_replacement)
        .returning(Replacement::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_replacement(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<ReplacementWithEmployees, Error> {
    replacement::table
        .inner_join(
            employee::table.on(employee::id
                .eq(replacement::original_employee_id)
                .and(employee::id.eq(replacement::replacement_employee_id))),
        )
        .select((
            replacement::all_columns,
            employee::first_name,
            employee::last_name,
            employee::first_name,
            employee::last_name,
        ))
        .filter(replacement::id.eq(id))
        .first::<(Replacement, String, String, String, String)>(conn)
        .map(
            |(
                replacement,
                original_first_name,
                original_last_name,
                replacement_first_name,
                replacement_last_name,
            )| ReplacementWithEmployees {
                replacement,
                original_employee_name: format!("{} {}", original_first_name, original_last_name),
                replacement_employee_name: format!(
                    "{} {}",
                    replacement_first_name, replacement_last_name
                ),
            },
        )
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_replacements(
    conn: &mut SqliteConnection,
) -> Result<Vec<ReplacementWithEmployees>, Error> {
    replacement::table
        .inner_join(
            employee::table.on(employee::id
                .eq(replacement::original_employee_id)
                .and(employee::id.eq(replacement::replacement_employee_id))),
        )
        .filter(replacement::replacement_end_date.ge(Local::now().naive_local().date()))
        .select((
            replacement::all_columns,
            employee::first_name,
            employee::last_name,
            employee::first_name,
            employee::last_name,
        ))
        .load::<(Replacement, String, String, String, String)>(conn)
        .map(|results| {
            results
                .into_iter()
                .map(
                    |(
                        replacement,
                        original_first_name,
                        original_last_name,
                        replacement_first_name,
                        replacement_last_name,
                    )| ReplacementWithEmployees {
                        replacement,
                        original_employee_name: format!(
                            "{} {}",
                            original_first_name, original_last_name
                        ),
                        replacement_employee_name: format!(
                            "{} {}",
                            replacement_first_name, replacement_last_name
                        ),
                    },
                )
                .collect()
        })
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_replacement(
    conn: &mut SqliteConnection,
    id: i32,
    original_employee_id: i32,
    replacement_employee_id: i32,
    replacement_start_date: NaiveDate,
    replacement_end_date: NaiveDate,
    assignment_id: i32,
    notes: Option<&str>,
) -> Result<Replacement, Error> {
    diesel::update(replacement::table.find(id))
        .set((
            replacement::original_employee_id.eq(original_employee_id),
            replacement::replacement_employee_id.eq(replacement_employee_id),
            replacement::replacement_start_date.eq(replacement_start_date),
            replacement::replacement_end_date.eq(replacement_end_date),
            replacement::assignment_id.eq(assignment_id),
            replacement::notes.eq(notes),
        ))
        .returning(Replacement::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_replacement(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(replacement::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
