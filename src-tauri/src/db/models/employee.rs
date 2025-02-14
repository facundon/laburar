use std::collections::HashMap;

use crate::db::models::employee_assignment::{EmployeeAssignment, EmployeeAssignmentWithNames};
use crate::db::schema::{
    absence, area, assignment, employee, employee_assignment, holiday, replacement, task,
};
use crate::error::Error;
use chrono::{Local, NaiveDate, NaiveDateTime, Utc};
use diesel::dsl::not;
use diesel::{prelude::*, RunQueryDsl, SqliteConnection};
use serde::{Deserialize, Serialize};

use super::replacement::list_replacements_for_assignment;

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
        .map_err(Error::Database)
}

pub fn get_employee(conn: &mut SqliteConnection, id: i32) -> Result<Employee, Error> {
    employee::table
        .find(id)
        .first(conn)
        .map_err(Error::Database)
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
            assignment::difficulty.nullable(),
            area::id.nullable(),
            area::name.nullable(),
            task::id.nullable(),
            task::name.nullable(),
        ))
        .order_by(employee_assignment::is_primary.desc())
        .then_order_by(assignment::difficulty.desc())
        .then_order_by(employee_assignment::efficiency.desc())
        .load(conn)
        .and_then(
            |results: Vec<(
                Employee,
                Option<EmployeeAssignment>,
                Option<i32>,
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

                for (
                    _,
                    employee_assignment,
                    assignment_difficulty,
                    area_id,
                    area_name,
                    task_id,
                    task_name,
                ) in results
                {
                    if let (
                        Some(employee_assignment),
                        Some(assignment_difficulty),
                        Some(area_id),
                        Some(area_name),
                        Some(task_id),
                        Some(task_name),
                    ) = (
                        employee_assignment,
                        assignment_difficulty,
                        area_id,
                        area_name,
                        task_id,
                        task_name,
                    ) {
                        employee_with_assignments.push(EmployeeAssignmentWithNames {
                            employee_assignment,
                            replacements: None,
                            assignment_difficulty,
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
        .map_err(Error::Database)
}

pub fn list_employees(conn: &mut SqliteConnection) -> Result<Vec<Employee>, Error> {
    employee::table.load(conn).map_err(Error::Database)
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
        .map_err(Error::Database)
}

pub fn delete_employee(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(employee::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeForAssignment {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub efficiency: i32,
    pub task_difficulty: i32,
    pub is_primary: bool,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub assignments_difficulties: Vec<i32>,
}

pub fn list_competent_employees_for_assignment(
    conn: &mut SqliteConnection,
    assignment_id: i32,
    assignation_start_date: NaiveDate,
    assignation_end_date: NaiveDate,
) -> Result<Vec<EmployeeForAssignment>, Error> {
    let mut results: Vec<EmployeeForAssignment> = employee::table
        .left_join(
            employee_assignment::table
                .on(employee::id.eq(employee_assignment::employee_id))
                .inner_join(assignment::table),
        )
        .left_join(holiday::table.on(employee::id.eq(holiday::employee_id)))
        .filter(
            employee_assignment::assignment_id
                .eq(assignment_id)
                .and(employee_assignment::is_primary.eq(false)),
        )
        .filter(
            holiday::start_date
                .is_null()
                .or(holiday::end_date.is_null())
                .or(not(holiday::start_date
                    .lt(assignation_start_date)
                    .and(holiday::end_date.gt(assignation_end_date)))),
        )
        .select((
            employee::id,
            employee::first_name,
            employee::last_name,
            assignment::difficulty.nullable(),
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
            Option<i32>,
            Option<bool>,
            Option<NaiveDate>,
            Option<NaiveDate>,
        )>(conn)
        .map(|results| {
            results
                .into_iter()
                .map(
                    |(
                        id,
                        first_name,
                        last_name,
                        task_difficulty,
                        efficiency,
                        is_primary,
                        start_date,
                        end_date,
                    )| {
                        EmployeeForAssignment {
                            id,
                            first_name,
                            last_name,
                            start_date,
                            end_date,
                            task_difficulty: task_difficulty.unwrap_or(0),
                            efficiency: efficiency.unwrap_or(0),
                            is_primary: is_primary.unwrap_or(false),
                            assignments_difficulties: vec![],
                        }
                    },
                )
                .collect()
        })
        .map_err(Error::Database)?;

    for employee in &mut results {
        if let Some(end_date) = employee.end_date {
            let pivot = if assignation_start_date > chrono::Local::now().date_naive() {
                assignation_start_date
            } else {
                chrono::Local::now().date_naive()
            };
            if end_date < pivot {
                employee.end_date = None;
                employee.start_date = None;
            }
        }

        let assignments_difficulties = assignment::table
            .inner_join(employee_assignment::table)
            .filter(
                employee_assignment::employee_id
                    .eq(employee.id)
                    .and(employee_assignment::is_primary),
            )
            .select(assignment::difficulty)
            .load(conn)
            .map_err(Error::Database);
        if let Ok(assignments_difficulties) = assignments_difficulties {
            employee.assignments_difficulties = assignments_difficulties;
        }
    }
    Ok(results)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeOnHoliday {
    #[serde(flatten)]
    pub employee: EmployeeWithAssignments,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_off: i32,
}

pub fn list_employees_future_absences(
    conn: &mut SqliteConnection,
) -> Result<Vec<EmployeeOnHoliday>, Error> {
    let today = Local::now().date_naive();
    employee::table
        .inner_join(absence::table)
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
        .filter(absence::absence_date.ge(today))
        .select((
            employee::all_columns,
            absence::absence_date,
            employee_assignment::all_columns.nullable(),
            assignment::difficulty.nullable(),
            area::id.nullable(),
            area::name.nullable(),
            task::id.nullable(),
            task::name.nullable(),
        ))
        .load::<(
            Employee,
            NaiveDate,
            Option<EmployeeAssignment>,
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<i32>,
            Option<String>,
        )>(conn)
        .map(|result| {
            let mut employees_map = HashMap::new();
            for (
                employee,
                start_date,
                employee_assignment,
                assignment_difficulty,
                area_id,
                area_name,
                task_id,
                task_name,
            ) in result
            {
                let entry = employees_map
                    .entry(employee.id)
                    .or_insert_with(|| EmployeeOnHoliday {
                        employee: EmployeeWithAssignments {
                            employee: employee.clone(),
                            assignments: vec![],
                        },
                        start_date,
                        end_date: start_date,
                        days_off: 1,
                    });

                if let (
                    Some(employee_assignment),
                    Some(assignment_difficulty),
                    Some(area_id),
                    Some(area_name),
                    Some(task_id),
                    Some(task_name),
                ) = (
                    employee_assignment,
                    assignment_difficulty,
                    area_id,
                    area_name,
                    task_id,
                    task_name,
                ) {
                    let replacements =
                        list_replacements_for_assignment(conn, employee_assignment.assignment_id)
                            .unwrap();
                    entry
                        .employee
                        .assignments
                        .push(EmployeeAssignmentWithNames {
                            employee_assignment,
                            replacements: Some(replacements),
                            assignment_difficulty,
                            area_id,
                            area_name,
                            task_id,
                            task_name,
                        });
                }
            }
            employees_map.into_values().collect::<Vec<_>>()
        })
        .map_err(Error::Database)
}

pub fn list_employees_on_holidays(
    conn: &mut SqliteConnection,
) -> Result<Vec<EmployeeOnHoliday>, Error> {
    let today = Local::now().date_naive();
    let employees_on_holidays = employee::table
        .inner_join(holiday::table)
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
        .filter(
            holiday::end_date
                .ge(today)
                .and(employee_assignment::is_primary.eq(true)),
        )
        .select((
            employee::all_columns,
            holiday::start_date,
            holiday::end_date,
            holiday::days_off,
            employee_assignment::all_columns.nullable(),
            assignment::difficulty.nullable(),
            area::id.nullable(),
            area::name.nullable(),
            task::id.nullable(),
            task::name.nullable(),
        ))
        .load::<(
            Employee,
            NaiveDate,
            NaiveDate,
            i32,
            Option<EmployeeAssignment>,
            Option<i32>,
            Option<i32>,
            Option<String>,
            Option<i32>,
            Option<String>,
        )>(conn)
        .map(|result| {
            let mut employees_map = HashMap::new();
            for (
                employee,
                start_date,
                end_date,
                days_off,
                employee_assignment,
                assignment_difficulty,
                area_id,
                area_name,
                task_id,
                task_name,
            ) in result
            {
                let entry = employees_map
                    .entry(employee.id)
                    .or_insert_with(|| EmployeeOnHoliday {
                        employee: EmployeeWithAssignments {
                            employee: employee.clone(),
                            assignments: vec![],
                        },
                        start_date,
                        end_date,
                        days_off,
                    });

                if let (
                    Some(employee_assignment),
                    Some(assignment_difficulty),
                    Some(area_id),
                    Some(area_name),
                    Some(task_id),
                    Some(task_name),
                ) = (
                    employee_assignment,
                    assignment_difficulty,
                    area_id,
                    area_name,
                    task_id,
                    task_name,
                ) {
                    let replacements =
                        list_replacements_for_assignment(conn, employee_assignment.assignment_id)
                            .unwrap();
                    entry
                        .employee
                        .assignments
                        .push(EmployeeAssignmentWithNames {
                            employee_assignment,
                            replacements: Some(replacements),
                            assignment_difficulty,
                            area_id,
                            area_name,
                            task_id,
                            task_name,
                        });
                }
            }
            employees_map.into_values().collect::<Vec<_>>()
        })
        .map_err(Error::Database)?;
    Ok(employees_on_holidays)
}

pub fn list_employees_replacing_assignment(
    conn: &mut SqliteConnection,
    assignment_id: i32,
) -> Result<Vec<i32>, Error> {
    let today = Utc::now().naive_utc().date();

    employee::table
        .inner_join(replacement::table.on(employee::id.eq(replacement::replacement_employee_id)))
        .filter(
            replacement::assignment_id
                .eq(assignment_id)
                .and(replacement::replacement_start_date.le(today))
                .and(replacement::replacement_end_date.ge(today)),
        )
        .order_by(replacement::replacement_start_date.asc())
        .select(employee::id)
        .load(conn)
        .map_err(Error::Database)
}
