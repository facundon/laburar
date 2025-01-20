use crate::db::models::{area::Area, task::Task};
use crate::db::schema::{area, assignment, task};
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Debug)]
#[diesel(table_name = assignment)]
#[diesel(belongs_to(Area))]
#[diesel(belongs_to(Task))]
#[diesel(primary_key(task_id, area_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Assignment {
    pub id: i32,
    pub task_id: i32,
    pub area_id: i32,
    pub difficulty: i32,
    pub frequency: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignmentWithNames {
    #[serde(flatten)]
    pub assignment: Assignment,
    pub task_name: String,
    pub area_name: String,
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

pub fn list_assignments(conn: &mut SqliteConnection) -> Result<Vec<AssignmentWithNames>, Error> {
    assignment::table
        .inner_join(task::table.on(task::id.eq(assignment::task_id)))
        .inner_join(area::table.on(area::id.eq(assignment::area_id)))
        .select((assignment::all_columns, task::name, area::name))
        .load(conn)
        .map(|assignments| {
            assignments
                .into_iter()
                .map(|(assignment, task_name, area_name)| AssignmentWithNames {
                    assignment,
                    task_name,
                    area_name,
                })
                .collect()
        })
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_assignment(conn: &mut SqliteConnection, id: i32) -> Result<AssignmentWithNames, Error> {
    assignment::table
        .inner_join(task::table.on(task::id.eq(assignment::task_id)))
        .inner_join(area::table.on(area::id.eq(assignment::area_id)))
        .select((assignment::all_columns, task::name, area::name))
        .filter(assignment::id.eq(id))
        .first(conn)
        .map(|(assignment, task_name, area_name)| AssignmentWithNames {
            assignment,
            task_name,
            area_name,
        })
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
