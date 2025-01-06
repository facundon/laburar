use chrono::NaiveDateTime;
use diesel::{RunQueryDsl, SelectableHelper, SqliteConnection, prelude::*};
use crate::db::schema::employee;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<i32>,
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
    pub phone: Option<&'a i32>,
    pub start_date: Option<&'a NaiveDateTime>,
}


pub fn create_employee(conn: &mut SqliteConnection, address: &str, first_name: &str, last_name: &str, phone: Option<&i32>, start_date: Option<&NaiveDateTime>) -> Employee {
    let new_employee = NewEmployee { address, first_name, last_name, phone, start_date};

    diesel::insert_into(employee::table)
        .values(&new_employee)
        .returning(Employee::as_returning())
        .get_result(conn)
        .expect("Error saving new employee")
}
