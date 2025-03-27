mod handlers;

use std::fs::OpenOptions;

use handlers::habit;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
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
        .invoke_handler(tauri::generate_handler![habit::get_habit])
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
