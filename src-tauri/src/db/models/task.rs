use crate::db::models::employee::Employee;
use crate::db::schema::{employee, employee_on_task, task};
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
    pub area: Option<String>,
    pub difficulty: String,
    pub frequency: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = task)]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub area: Option<&'a str>,
    pub difficulty: &'a str,
    pub frequency: &'a str,
}

pub fn create_task(
    conn: &mut SqliteConnection,
    name: &str,
    description: Option<&str>,
    area: Option<&str>,
    difficulty: &str,
    frequency: &str,
) -> Result<Task, Error> {
    let new_task = NewTask {
        name,
        description,
        area,
        difficulty,
        frequency,
    };

    diesel::insert_into(task::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_task(conn: &mut SqliteConnection, id: i32) -> Result<Task, Error> {
    task::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_tasks(conn: &mut SqliteConnection) -> Result<Vec<Task>, Error> {
    task::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_task(
    conn: &mut SqliteConnection,
    id: i32,
    name: &str,
    description: Option<&str>,
    area: Option<&str>,
    difficulty: &str,
    frequency: &str,
) -> Result<Task, Error> {
    diesel::update(task::table.find(id))
        .set((
            task::name.eq(name),
            task::description.eq(description),
            task::area.eq(area),
            task::difficulty.eq(difficulty),
            task::frequency.eq(frequency),
        ))
        .returning(Task::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_task(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(task::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}

pub fn get_task_with_employees(
    conn: &mut SqliteConnection,
    task_id: i32,
) -> Result<(Task, Vec<Employee>), Error> {
    let task = get_task(conn, task_id)?;
    let employees = employee::table
        .inner_join(employee_on_task::table.on(employee_on_task::employee_id.eq(employee::id)))
        .inner_join(task::table.on(task::id.eq(employee_on_task::task_id)))
        .filter(task::id.eq(task_id))
        .select(employee::all_columns)
        .load::<Employee>(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok((task, employees))
}

pub fn get_tasks_for_employee(
    conn: &mut SqliteConnection,
    employee_id: i32,
) -> Result<Vec<Task>, Error> {
    task::table
        .inner_join(employee_on_task::table.on(employee_on_task::task_id.eq(task::id)))
        .inner_join(employee::table.on(employee::id.eq(employee_on_task::employee_id)))
        .filter(employee::id.eq(employee_id))
        .select(task::all_columns)
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}
