use crate::db::schema::employee_assignment;
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = employee_assignment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EmployeeAssignment {
    id: i32,
    employee_id: i32,
    assignment_id: i32,
    is_primary: Option<bool>,
    efficiency: i32,
    assigned_date: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = employee_assignment)]
pub struct NewEmployeeAssignment<'a> {
    pub employee_id: i32,
    pub assignment_id: i32,
    pub is_primary: Option<bool>,
    pub efficiency: i32,
    pub assigned_date: Option<&'a NaiveDateTime>,
}

pub fn list_employee_assignments(
    conn: &mut SqliteConnection,
    employee_id: i32,
) -> Result<Vec<EmployeeAssignment>, Error> {
    let assignments = employee_assignment::table
        .filter(employee_assignment::employee_id.eq(employee_id))
        .select(employee_assignment::all_columns)
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(assignments)
}

pub fn create_assignments_to_employee(
    conn: &mut SqliteConnection,
    employee_id: i32,
    assigment_ids: Vec<i32>,
) -> Result<(), Error> {
    for assignment_id in assigment_ids {
        let new_assignment = NewEmployeeAssignment {
            employee_id,
            assignment_id,
            is_primary: None,
            efficiency: 0,
            assigned_date: None,
        };
        diesel::insert_into(employee_assignment::table)
            .values(&new_assignment)
            .execute(conn)
            .map_err(|err| Error::Database(err.to_string()))?;
    }
    Ok(())
}

pub fn update_employee_assignment(
    conn: &mut SqliteConnection,
    employee_id: i32,
    assignment_id: i32,
    efficiency: i32,
    is_primary: bool,
) -> Result<(), Error> {
    diesel::update(
        employee_assignment::table.filter(
            employee_assignment::employee_id
                .eq(employee_id)
                .and(employee_assignment::assignment_id.eq(assignment_id)),
        ),
    )
    .set((
        employee_assignment::is_primary.eq(is_primary),
        employee_assignment::efficiency.eq(efficiency),
    ))
    .execute(conn)
    .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}

pub fn delete_employee_assignment(
    conn: &mut SqliteConnection,
    employee_id: i32,
    assignment_id: i32,
) -> Result<(), Error> {
    diesel::delete(
        employee_assignment::table.filter(
            employee_assignment::employee_id
                .eq(employee_id)
                .and(employee_assignment::assignment_id.eq(assignment_id)),
        ),
    )
    .execute(conn)
    .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
