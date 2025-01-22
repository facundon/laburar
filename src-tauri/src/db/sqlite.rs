use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs::data_dir;
use dotenvy::dotenv;
use std::{env, error::Error};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn get_dev_db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn get_prod_db_url() -> String {
    let mut data_path = data_dir().unwrap();
    data_path.push("LaburAR");
    std::fs::create_dir_all(&data_path).expect("Failed to create LaburAR directory");

    let path = data_path.join("database.db");
    if !path.exists() {
        std::fs::File::create(&path).expect("Failed to create database file");
    }
    path.to_str()
        .expect("Failed to convert path to string")
        .to_string()
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url: String = if cfg!(dev) {
        get_dev_db_url()
    } else {
        get_prod_db_url()
    };

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
