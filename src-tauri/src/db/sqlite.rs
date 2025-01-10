use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::{env, error::Error};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url: String = if cfg!(debug_assertions) {
        // Development environment
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        // Production environment
        let mut path = std::path::PathBuf::from(tauri::path::BaseDirectory::Data.variable());
        path.push("database.db");
        path.to_str()
            .expect("Failed to convert path to string")
            .to_string()
    };

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = establish_connection();
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
