use crate::db::models::task::Task;
use crate::db::schema::{employee, employee_on_task, task};
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub address: String,
    pub start_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub address: &'a str,
    pub phone: Option<&'a str>,
    pub start_date: Option<&'a NaiveDateTime>,
}

pub fn create_employee(
    conn: &mut SqliteConnection,
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<&NaiveDateTime>,
) -> Result<Employee, Error> {
    let new_employee = NewEmployee {
        address,
        first_name,
        last_name,
        phone,
        start_date,
    };

    diesel::insert_into(employee::table)
        .values(&new_employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_employee(conn: &mut SqliteConnection, id: i32) -> Result<Employee, Error> {
    employee::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_employees(conn: &mut SqliteConnection) -> Result<Vec<Employee>, Error> {
    employee::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_employee(
    conn: &mut SqliteConnection,
    id: i32,
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<&NaiveDateTime>,
) -> Result<Employee, Error> {
    diesel::update(employee::table.find(id))
        .set((
            employee::address.eq(address),
            employee::first_name.eq(first_name),
            employee::last_name.eq(last_name),
            employee::phone.eq(phone),
            employee::start_date.eq(start_date),
        ))
        .returning(Employee::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_employee(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(employee::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}

pub fn get_employee_with_tasks(
    conn: &mut SqliteConnection,
    employee_id: i32,
) -> Result<(Employee, Vec<Task>), Error> {
    let employee = get_employee(conn, employee_id)?;
    let tasks = task::table
        .inner_join(employee_on_task::table.on(employee_on_task::task_id.eq(task::id)))
        .inner_join(employee::table.on(employee::id.eq(employee_on_task::employee_id)))
        .filter(employee::id.eq(employee_id))
        .select(task::all_columns)
        .load::<Task>(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok((employee, tasks))
}

pub fn assign_tasks_to_employee(
    conn: &mut SqliteConnection,
    employee_id: i32,
    task_ids: Vec<i32>,
) -> Result<(), Error> {
    let new_assignments = task_ids
        .iter()
        .map(|task_id| {
            (
                employee_on_task::employee_id.eq(employee_id),
                employee_on_task::task_id.eq(*task_id),
            )
        })
        .collect::<Vec<_>>();

    diesel::insert_into(employee_on_task::table)
        .values(&new_assignments)
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
