use crate::db::schema::{employee, holiday};
use crate::error::Error;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use diesel::{prelude::*, RunQueryDsl, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = holiday)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Holiday {
    pub id: i32,
    pub employee_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_off: i32,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HolidayWithEmployee {
    #[serde(flatten)]
    pub holiday: Holiday,
    pub employee_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = holiday)]
pub struct NewHoliday<'a> {
    pub employee_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub notes: Option<&'a str>,
}

#[derive(Insertable)]
#[diesel(table_name = holiday)]
pub struct NewHolidayWithDaysOff<'a> {
    #[diesel(embed)]
    pub base: NewHoliday<'a>,
    pub days_off: i32,
}

impl<'a> From<NewHoliday<'a>> for NewHolidayWithDaysOff<'a> {
    fn from(base: NewHoliday<'a>) -> Self {
        let days_off = base
            .end_date
            .signed_duration_since(base.start_date)
            .num_days()
            .saturating_add(1) as i32;
        Self { base, days_off }
    }
}

pub fn create_holiday(
    conn: &mut SqliteConnection,
    employee_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    let base_holiday = NewHoliday {
        employee_id,
        start_date,
        end_date,
        notes,
    };

    let new_holiday = NewHolidayWithDaysOff::from(base_holiday);

    conn.transaction(|conn| {
        let days_off = new_holiday.days_off;
        diesel::update(employee::table.find(employee_id))
            .set(employee::accumulated_holidays.eq(employee::accumulated_holidays.sub(days_off)))
            .execute(conn)?;

        diesel::insert_into(holiday::table)
            .values(&new_holiday)
            .returning(Holiday::as_returning())
            .get_result(conn)
    })
    .map_err(Error::Database)
}

pub fn get_holiday(conn: &mut SqliteConnection, id: i32) -> Result<HolidayWithEmployee, Error> {
    holiday::table
        .inner_join(employee::table)
        .select((
            holiday::all_columns,
            employee::first_name,
            employee::last_name,
        ))
        .filter(holiday::id.eq(id))
        .first::<(Holiday, String, String)>(conn)
        .map(|(holiday, first_name, last_name)| HolidayWithEmployee {
            holiday,
            employee_name: format!("{} {}", first_name, last_name),
        })
        .map_err(Error::Database)
}

pub fn list_holidays(
    conn: &mut SqliteConnection,
    year: Option<i32>,
) -> Result<Vec<HolidayWithEmployee>, Error> {
    let current_year = Local::now().year();
    let year = year.unwrap_or(current_year);
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    holiday::table
        .inner_join(employee::table)
        .select((
            holiday::all_columns,
            employee::first_name,
            employee::last_name,
        ))
        .filter(
            holiday::start_date
                .ge(start_date)
                .and(holiday::end_date.le(end_date)),
        )
        .load::<(Holiday, String, String)>(conn)
        .map(|results| {
            results
                .into_iter()
                .map(|(holiday, first_name, last_name)| HolidayWithEmployee {
                    holiday,
                    employee_name: format!("{} {}", first_name, last_name),
                })
                .collect()
        })
        .map_err(Error::Database)
}

pub fn update_holiday(
    conn: &mut SqliteConnection,
    id: i32,
    employee_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    notes: Option<&str>,
) -> Result<Holiday, Error> {
    let base_holiday = NewHoliday {
        employee_id,
        start_date,
        end_date,
        notes,
    };
    let new_holiday = NewHolidayWithDaysOff::from(base_holiday);

    conn.transaction(|conn| {
        let new_days_off = new_holiday.days_off;
        let old_holiday = holiday::table
            .find(id)
            .select(holiday::days_off)
            .first::<i32>(conn)
            .map_err(Error::Database);
        let old_days_off = old_holiday.unwrap();
        let difference = old_days_off - new_days_off;

        diesel::update(employee::table.find(employee_id))
            .set(employee::accumulated_holidays.eq(employee::accumulated_holidays.add(difference)))
            .execute(conn)?;

        diesel::update(holiday::table.find(id))
            .set((
                holiday::employee_id.eq(employee_id),
                holiday::start_date.eq(start_date),
                holiday::end_date.eq(end_date),
                holiday::days_off.eq(new_days_off),
                holiday::notes.eq(notes),
            ))
            .returning(Holiday::as_returning())
            .get_result(conn)
    })
    .map_err(Error::Database)
}

pub fn delete_holiday(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    conn.transaction(|conn| {
        let holiday = holiday::table
            .find(id)
            .select((holiday::days_off, holiday::employee_id))
            .first::<(i32, i32)>(conn);
        let (days_off, employee_id) = holiday.unwrap();
        diesel::delete(holiday::table.find(id)).execute(conn)?;
        diesel::update(employee::table.find(employee_id))
            .set(employee::accumulated_holidays.eq(employee::accumulated_holidays.add(days_off)))
            .execute(conn)?;
        Ok(())
    })
    .map_err(Error::Database)
}

pub fn list_holidays_for_employee(
    conn: &mut SqliteConnection,
    employee_id: i32,
) -> Result<Vec<Holiday>, Error> {
    holiday::table
        .filter(holiday::employee_id.eq(employee_id))
        .order_by(holiday::start_date.desc())
        .load::<Holiday>(conn)
        .map_err(Error::Database)
}
