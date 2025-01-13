use crate::db::schema::area;
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::area)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = area)]
pub struct NewArea<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

pub fn create_area(
    conn: &mut SqliteConnection,
    name: &str,
    description: Option<&str>,
) -> Result<Area, Error> {
    let new_area = NewArea { name, description };

    diesel::insert_into(area::table)
        .values(&new_area)
        .returning(Area::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn get_area(conn: &mut SqliteConnection, id: i32) -> Result<Area, Error> {
    area::table
        .find(id)
        .first(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn list_areas(conn: &mut SqliteConnection) -> Result<Vec<Area>, Error> {
    area::table
        .load(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn update_area(
    conn: &mut SqliteConnection,
    id: i32,
    name: &str,
    description: Option<&str>,
) -> Result<Area, Error> {
    diesel::update(area::table.find(id))
        .set((area::name.eq(name), area::description.eq(description)))
        .returning(Area::as_returning())
        .get_result(conn)
        .map_err(|err| Error::Database(err.to_string()))
}

pub fn delete_area(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(area::table.find(id))
        .execute(conn)
        .map_err(|err| Error::Database(err.to_string()))?;
    Ok(())
}
