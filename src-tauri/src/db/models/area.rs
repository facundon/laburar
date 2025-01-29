use crate::db::models::assignment::{Assignment, AssignmentWithNames};
use crate::db::schema::{area, assignment, task};
use crate::error::Error;
use chrono::NaiveDateTime;
use diesel::{prelude::*, SelectableHelper, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Default, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = area)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Area {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaWithAssignments {
    #[serde(flatten)]
    pub area: Area,
    pub assignments: Vec<AssignmentWithNames>,
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
        .map_err(Error::Database)
}

pub fn get_area(conn: &mut SqliteConnection, id: i32) -> Result<Area, Error> {
    area::table.find(id).first(conn).map_err(Error::Database)
}

pub fn list_areas(conn: &mut SqliteConnection) -> Result<Vec<Area>, Error> {
    area::table.load(conn).map_err(Error::Database)
}

pub fn get_area_with_assignments(
    conn: &mut SqliteConnection,
    area_id: i32,
) -> Result<AreaWithAssignments, Error> {
    area::table
        .left_join(
            assignment::table
                .inner_join(task::table.on(task::id.eq(assignment::task_id)))
                .on(assignment::area_id.eq(area::id)),
        )
        .select((
            area::all_columns,
            Option::<Assignment>::as_select(),
            task::name.nullable(),
        ))
        .filter(area::id.eq(area_id))
        .load::<(Area, Option<Assignment>, Option<String>)>(conn)
        .and_then(|results| {
            if results.is_empty() {
                return Err(diesel::result::Error::NotFound);
            }
            let mut assignments = vec![];
            let area = results[0].0.clone();
            for (_, assignment, task_name) in results {
                if let (Some(assignment), Some(task_name)) = (assignment, task_name) {
                    assignments.push(AssignmentWithNames {
                        assignment,
                        area_name: area.name.clone(),
                        task_name,
                    });
                }
            }
            Ok(AreaWithAssignments { area, assignments })
        })
        .map_err(Error::Database)
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
        .map_err(Error::Database)
}

pub fn delete_area(conn: &mut SqliteConnection, id: i32) -> Result<(), Error> {
    diesel::delete(area::table.find(id))
        .execute(conn)
        .map_err(Error::Database)?;
    Ok(())
}
