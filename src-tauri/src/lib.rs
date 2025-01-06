mod commands;
mod db;

use commands::employee::create_employee_command;
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
        .invoke_handler(tauri::generate_handler![create_employee_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
