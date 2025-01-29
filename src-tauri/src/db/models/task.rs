use crate::db::schema::{assignment, task};
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::task)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

pub fn create_task(
    conn: &mut SqliteConnection,
    name: &str,
    description: Option<&str>,
) -> Result<Task, Error> {
    let new_task = NewTask { name, description };

    diesel::insert_into(task::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn get_task(conn: &mut SqliteConnection, id: i32) -> Result<Task, Error> {
    task::table.find(id).first(conn).map_err(Error::Database)
}

pub fn list_tasks(conn: &mut SqliteConnection, exclude_ids: Vec<i32>) -> Result<Vec<Task>, Error> {
    task::table
        .filter(task::id.ne_all(&exclude_ids))
        .load(conn)
        .map_err(Error::Database)
}

pub fn update_task(
    conn: &mut SqliteConnection,
    id: i32,
    name: &str,
    description: Option<&str>,
) -> Result<Task, Error> {
    diesel::update(task::table.find(id))
        .set((task::name.eq(name), task::description.eq(description)))
        .returning(Task::as_returning())
        .get_result(conn)
        .map_err(Error::Database)
}

pub fn delete_task(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(task::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}

pub fn get_tasks_for_area(conn: &mut SqliteConnection, area_id: i32) -> Result<Vec<Task>, Error> {
    task::table
        .inner_join(assignment::table.on(task::id.eq(assignment::task_id)))
        .filter(assignment::area_id.eq(area_id))
        .select(task::all_columns)
        .load::<Task>(conn)
        .map_err(Error::Database)
}
