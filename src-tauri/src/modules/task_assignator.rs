use ::serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use diesel::SqliteConnection;

use crate::{
    db::models::employee::{list_competent_employees_for_assignment, EmployeeForAssignment},
    error::Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum SuggestionResult {
    Employees(Vec<EmployeeWithScore>),
    Message(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct EmployeeWithScore {
    #[serde(flatten)]
    pub employee: EmployeeForAssignment,
    pub score: f32,
}

pub fn sugest_employees_for_assignation(
    conn: &mut SqliteConnection,
    assignment_id: i32,
    assignation_start_date: NaiveDate,
    assignation_end_date: NaiveDate,
) -> Result<SuggestionResult, Error> {
    let employees = list_competent_employees_for_assignment(conn, assignment_id).unwrap();
    if employees.len() == 0 {
        return Err(Error::Assignator(SuggestionResult::Message(
            "No employees available for this task.".to_string(),
        )));
    }
    println!("{:#?}", employees);

    let mut employees_with_scores: Vec<EmployeeWithScore> = Vec::new();
    for employee in employees {
        let total_task_difficulty = employee.assignments_difficulties.iter().sum::<i32>();
        let mut score =
            (employee.efficiency - employee.task_difficulty / (1 + total_task_difficulty)) as f32;

        let assignation_duration =
            (assignation_end_date - assignation_start_date).num_days() as i32;
        if assignation_duration < 0 {
            return Err(Error::Assignator(SuggestionResult::Message(
                "The assignation duration is invalid.".to_string(),
            )));
        }

        let holiday_start_date = employee.start_date;
        if let Some(holiday_start_date) = holiday_start_date {
            let days_until_vacation =
                (holiday_start_date - Utc::now().naive_utc().date()).num_days() as i32;
            if days_until_vacation < assignation_duration {
                score -=
                    ((assignation_duration - days_until_vacation) / assignation_duration) as f32;
            }
        }
        employees_with_scores.push(EmployeeWithScore { employee, score });
    }
    employees_with_scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
    Ok(SuggestionResult::Employees(employees_with_scores))
}
