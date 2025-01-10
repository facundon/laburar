mod commands;
mod db;
mod error;

use commands::employee::{
    assign_tasks_to_employee_command, create_employee_command, delete_employee_command,
    get_employee_command, get_employee_with_tasks_command, list_employees_command,
    update_employee_command,
};
use commands::task::{
    create_task_command, delete_task_command, get_task_command, get_task_with_employees_command,
    get_tasks_for_employee_command, list_tasks_command, update_task_command,
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
            list_employees_command,
            update_employee_command,
            create_task_command,
            delete_task_command,
            get_task_command,
            list_tasks_command,
            update_task_command,
            get_employee_with_tasks_command,
            get_task_with_employees_command,
            assign_tasks_to_employee_command,
            get_tasks_for_employee_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
