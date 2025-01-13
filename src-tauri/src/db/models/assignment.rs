use crate::db::schema::assignment;
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::assignment)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Assignment {
    pub id: i32,
    pub task_id: i32,
    pub area_id: i32,
    pub difficulty: i32,
    pub frequency: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = assignment)]
pub struct NewAssignment<'a> {
    pub task_id: i32,
    pub area_id: i32,
    pub difficulty: i32,
    pub frequency: &'a str,
}

pub fn create_assignment(
    conn: &mut SqliteConnection,
    task_id: i32,
    area_id: i32,
    difficulty: i32,
    frequency: &str,
) -> Result<Assignment, Error> {
    let new_assignment = NewAssignment {
        area_id,
        task_id,
        difficulty,
        frequency,
    };

    diesel::insert_into(assignment::table)
        .values(&new_assignment)
        .returning(Assignment::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_assignment(conn: &mut SqliteConnection, id: i32) -> Result<Assignment, Error> {
    assignment::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_assignments(conn: &mut SqliteConnection) -> Result<Vec<Assignment>, Error> {
    assignment::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_assignment(
    conn: &mut SqliteConnection,
    id: i32,
    difficulty: i32,
    frequency: &str,
) -> Result<Assignment, Error> {
    diesel::update(assignment::table.find(id))
        .set((
            assignment::difficulty.eq(difficulty),
            assignment::frequency.eq(frequency),
        ))
        .returning(Assignment::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_assignment(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(assignment::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
