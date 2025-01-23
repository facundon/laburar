use crate::db::models::employee_assignment::{EmployeeAssignment, EmployeeAssignmentWithNames};
use crate::db::schema::{area, assignment, employee, employee_assignment, holiday, task};
use crate::error::Error;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::dsl::not;
use diesel::{prelude::*, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = employee)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub address: String,
    pub start_date: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeWithAssignments {
    #[serde(flatten)]
    pub employee: Employee,
    pub assignments: Vec<EmployeeAssignmentWithNames>,
}

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub address: &'a str,
    pub phone: Option<&'a str>,
    pub start_date: Option<&'a NaiveDate>,
}

pub fn create_employee(
    conn: &mut SqliteConnection,
    address: &str,
    first_name: &str,
    last_name: &str,
    phone: Option<&str>,
    start_date: Option<&NaiveDate>,
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

pub fn get_employee_with_assignments(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<EmployeeWithAssignments, Error> {
    employee::table
        .left_join(
            employee_assignment::table
                .inner_join(
                    assignment::table
                        .on(assignment::id.eq(employee_assignment::assignment_id))
                        .inner_join(task::table.on(task::id.eq(assignment::task_id)))
                        .inner_join(area::table.on(area::id.eq(assignment::area_id))),
                )
                .on(employee::id.eq(employee_assignment::employee_id)),
        )
        .filter(employee::id.eq(id))
        .select((
            employee::all_columns,
            employee_assignment::all_columns.nullable(),
            area::id.nullable(),
            area::name.nullable(),
            task::id.nullable(),
            task::name.nullable(),
        ))
        .load(conn)
        .and_then(
            |results: Vec<(
                Employee,
                Option<EmployeeAssignment>,
                Option<i32>,
                Option<String>,
                Option<i32>,
                Option<String>,
            )>| {
                if results.is_empty() {
                    return Err(diesel::result::Error::NotFound);
                }
                let employee = results[0].0.clone();
                let mut employee_with_assignments = vec![];

                for (_, employee_assignment, area_id, area_name, task_id, task_name) in results {
                    if let (
                        Some(employee_assignment),
                        Some(area_id),
                        Some(area_name),
                        Some(task_id),
                        Some(task_name),
                    ) = (employee_assignment, area_id, area_name, task_id, task_name)
                    {
                        employee_with_assignments.push(EmployeeAssignmentWithNames {
                            employee_assignment,
                            area_id,
                            area_name,
                            task_id,
                            task_name,
                        });
                    }
                }
                Ok(EmployeeWithAssignments {
                    employee,
                    assignments: employee_with_assignments,
                })
            },
        )
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
    start_date: Option<&NaiveDate>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeForAssignment {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub efficiency: i32,
    pub is_primary: bool,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub assignments_difficulties: Vec<i32>,
}

pub fn list_competent_employees_for_assignment(
    conn: &mut SqliteConnection,
    assignment_id: i32,
) -> Result<Vec<EmployeeForAssignment>, Error> {
    let today = Utc::now().naive_utc().date();

    let mut results: Vec<EmployeeForAssignment> = employee::table
        .left_join(employee_assignment::table.on(employee::id.eq(employee_assignment::employee_id)))
        .left_join(holiday::table.on(employee::id.eq(holiday::employee_id)))
        .filter(employee_assignment::assignment_id.eq(assignment_id))
        .filter(
            not(holiday::start_date
                .le(today)
                .and(holiday::end_date.ge(today)))
            .or(holiday::start_date
                .is_null()
                .and(holiday::end_date.is_null())),
        )
        .select((
            employee::id,
            employee::first_name,
            employee::last_name,
            employee_assignment::efficiency.nullable(),
            employee_assignment::is_primary.nullable(),
            holiday::start_date.nullable(),
            holiday::end_date.nullable(),
        ))
        .load::<(
            i32,
            String,
            String,
            Option<i32>,
            Option<bool>,
            Option<NaiveDate>,
            Option<NaiveDate>,
        )>(conn)
        .map(|results| {
            results
                .into_iter()
                .map(
                    |(id, first_name, last_name, efficiency, is_primary, start_date, end_date)| {
                        EmployeeForAssignment {
                            id,
                            first_name,
                            last_name,
                            start_date,
                            end_date,
                            efficiency: efficiency.unwrap_or(0),
                            is_primary: is_primary.unwrap_or(false),
                            assignments_difficulties: vec![],
                        }
                    },
                )
                .collect()
        })
        .map_err(|err| Error::Database(err.to_string()))?;

    for employee in &mut results {
        let assignments_difficulties = assignment::table
            .inner_join(employee_assignment::table)
            .filter(employee_assignment::employee_id.eq(employee.id))
            .select(assignment::difficulty)
            .load(conn)
            .map_err(|err| Error::Database(err.to_string()));
        if let Ok(assignments_difficulties) = assignments_difficulties {
            employee.assignments_difficulties = assignments_difficulties;
        }
    }
    Ok(results)
}
