mod commands;
mod db;
mod error;
mod utils;

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
use commands::employee::{
    create_employee_command, delete_employee_command, get_employee_command,
    get_employee_with_assignments_command, list_employees_command, update_employee_command,
};
use commands::employee_assignment::{
    create_assignments_to_employee_command, delete_employee_assignment_command,
    list_employee_assignments_command, update_employee_assignment_command,
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
            list_absences_for_employee_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
