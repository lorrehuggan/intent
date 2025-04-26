mod handlers;
mod models;

use handlers::app::{commit, habit, timeline};
use handlers::user::settings;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::fs::OpenOptions;
use tauri::{App, Manager};

struct AppState {
    db: Pool<Sqlite>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                let db = init_db(app).await;
                let state = AppState { db };
                app.manage(state);
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            habit::get_habit,
            habit::get_all_habits,
            habit::create_habit,
            habit::update_habit,
            habit::delete_habit,
            commit::create_commit,
            commit::delete_commit,
            commit::get_habit_commits,
            timeline::create_year_timeline,
            settings::get_user_settings,
            settings::set_user_settings,
            settings::set_theme,
            settings::set_highlight_current_day
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn init_db(app: &App) -> Pool<Sqlite> {
    let mut path = app
        .path()
        .app_local_data_dir()
        .expect("Error getting app data directory");

    path.push("db.sqlite");

    let result = OpenOptions::new().create_new(true).write(true).open(&path);

    match result {
        Ok(_) => println!("Database created at: {:?}", path),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => println!("Database already exists at: {:?}", path),
            std::io::ErrorKind::NotFound => println!("Database not found at: {:?}", path),
            _ => panic!("Error creating database: {:?}", e),
        },
    }

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .expect("Error connecting to database");

    println!("Migrating database, {}", path.to_str().unwrap());
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Error migrating database");
    println!("Database migrated");

    db
}
