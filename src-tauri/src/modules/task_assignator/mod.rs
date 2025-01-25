use ::serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use diesel::SqliteConnection;

use crate::{
    db::models::employee::{list_competent_employees_for_assignment, EmployeeForAssignment},
    error::Error,
};

// #[cfg(test)]
// mod tests;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmployeeWithScore {
    #[serde(flatten)]
    pub employee: EmployeeForAssignment,
    pub score: f32,
}

pub fn suggest_employees_for_assignation(
    conn: &mut SqliteConnection,
    assignment_id: i32,
    assignation_start_date: NaiveDate,
    assignation_end_date: NaiveDate,
) -> Result<Vec<EmployeeWithScore>, Error> {
    let employees = list_competent_employees_for_assignment(conn, assignment_id).unwrap();
    if employees.len() == 0 {
        return Ok(Vec::new());
    }

    let mut employees_with_scores: Vec<EmployeeWithScore> = Vec::new();
    for employee in employees {
        let total_task_difficulty = employee.assignments_difficulties.iter().sum::<i32>();
        let mut score: f32 =
            employee.efficiency as f32 + (1.0 / (1 + total_task_difficulty) as f32);

        let assignation_duration =
            (assignation_end_date - assignation_start_date).num_days() as i32;
        if assignation_duration < 0 {
            return Ok(Vec::new());
        }

        let holiday_start_date = employee.start_date;
        if let Some(holiday_start_date) = holiday_start_date {
            let days_until_vacation =
                (holiday_start_date - Utc::now().naive_utc().date()).num_days() as i32;
            if days_until_vacation < assignation_duration {
                score -= (assignation_duration - days_until_vacation) as f32
                    / assignation_duration as f32;
            }
        }
        employees_with_scores.push(EmployeeWithScore { employee, score });
    }
    employees_with_scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    Ok(employees_with_scores)
}
