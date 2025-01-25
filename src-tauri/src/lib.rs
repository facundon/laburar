mod commands;
mod db;
mod error;
mod modules;
mod utils;

use commands::task_assignator::{
    list_assignments_without_employees_command, suggest_employees_for_assignation_command,
};

use commands::absence::{
    create_absence_command, delete_absence_command, get_absence_command,
    get_absence_with_returns_command, list_absences_command, list_absences_for_employee_command,
    update_absence_command,
};
use commands::absence_return::{
    create_absence_return_command, delete_absence_return_command, get_absence_return_command,
    list_absence_returns_command, update_absence_return_command,
};
use commands::area::{
    create_area_command, delete_area_command, get_area_command, get_area_with_assignments_command,
    list_areas_command, update_area_command,
};
use commands::assignment::{
    create_assignment_command, delete_assignment_command, get_assignment_command,
    list_assignments_command, update_assignment_command,
};
use commands::company_holiday::{
    create_company_holiday_command, delete_company_holiday_command, get_company_holiday_command,
    list_company_holidays_command, update_company_holiday_command,
};
use commands::employee::{
    create_employee_command, delete_employee_command, get_employee_command,
    get_employee_with_assignments_command, list_employees_command,
    list_employees_on_holidays_command, update_employee_command,
};
use commands::employee_assignment::{
    create_assignments_to_employee_command, delete_employee_assignment_command,
    list_employee_assignments_command, update_employee_assignment_command,
};
use commands::holiday::{
    create_holiday_command, delete_holiday_command, get_holiday_command, list_holidays_command,
    update_holiday_command,
};
use commands::replacement::{
    create_replacement_command, delete_replacement_command, get_replacement_command,
    list_replacements_command, update_replacement_command,
};
use commands::task::{
    create_task_command, delete_task_command, get_task_command, get_tasks_for_area_command,
    list_tasks_command, update_task_command,
};
use db::sqlite::run_migrations;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Call run_migrations during setup
            run_migrations().expect("Failed to run migrations");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            suggest_employees_for_assignation_command,
            list_assignments_without_employees_command,
            list_employees_on_holidays_command,
            create_employee_command,
            delete_employee_command,
            get_employee_command,
            get_employee_with_assignments_command,
            list_employees_command,
            update_employee_command,
            create_task_command,
            delete_task_command,
            get_task_command,
            get_tasks_for_area_command,
            list_tasks_command,
            update_task_command,
            create_area_command,
            delete_area_command,
            get_area_command,
            get_area_with_assignments_command,
            list_areas_command,
            update_area_command,
            create_assignment_command,
            list_assignments_command,
            delete_assignment_command,
            get_assignment_command,
            update_assignment_command,
            create_assignments_to_employee_command,
            delete_employee_assignment_command,
            list_employee_assignments_command,
            update_employee_assignment_command,
            create_absence_command,
            delete_absence_command,
            get_absence_command,
            get_absence_with_returns_command,
            list_absences_command,
            update_absence_command,
            create_absence_return_command,
            delete_absence_return_command,
            get_absence_return_command,
            list_absence_returns_command,
            update_absence_return_command,
            list_absences_for_employee_command,
            create_holiday_command,
            delete_holiday_command,
            get_holiday_command,
            list_holidays_command,
            update_holiday_command,
            create_company_holiday_command,
            delete_company_holiday_command,
            get_company_holiday_command,
            list_company_holidays_command,
            update_company_holiday_command,
            create_replacement_command,
            delete_replacement_command,
            get_replacement_command,
            list_replacements_command,
            update_replacement_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
