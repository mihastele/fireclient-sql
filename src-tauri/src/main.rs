// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use sqlx::{Column, MySqlPool, PgPool, Row};

#[derive(Deserialize)]
struct ConnectionConfig {
    db_type: String, // "mysql", "postgres", "mariadb"
    host: String,
    port: u16,
    user: String,
    password: Option<String>,
    database: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct DbRow(Vec<String>);

#[derive(Debug, Serialize)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<DbRow>,
}

// --- Connect to Database ---
#[tauri::command]
async fn connect_to_db(config: ConnectionConfig) -> Result<String, String> {
    match config.db_type.as_str() {
        "mysql" | "mariadb" => {
            let url = format!(
                "mysql://{}:{}@{}:{}/{}",
                config.user,
                config.password.unwrap_or_default(),
                config.host,
                config.port,
                config.database
            );
            match MySqlPool::connect(&url).await {
                Ok(_) => Ok("Successfully connected to MySQL/MariaDB!".into()),
                Err(e) => Err(format!("MySQL connection error: {}", e)),
            }
        }
        "postgres" => {
            let url = format!(
                "postgres://{}:{}@{}:{}/{}",
                config.user,
                config.password.unwrap_or_default(),
                config.host,
                config.port,
                config.database
            );

            match PgPool::connect(&url).await {
                Ok(_) => Ok("Successfully connected to Postgres!".into()),
                Err(e) => Err(format!("Postgres connection error: {}", e)),
            }
        }
        _ => Err("Unsupported database type".into()),
    }
}
#[tauri::command]
async fn execute_query(config: ConnectionConfig, query: String) -> Result<QueryResult, String> {
    match config.db_type.as_str() {
        "mysql" | "mariadb" => execute_mysql_query(config, query).await,
        "postgres" => execute_postgres_query(config, query).await,
        _ => Err("Unsupported database type for query execution".into()),
    }
}
async fn execute_mysql_query(
    config: ConnectionConfig,
    query: String,
) -> Result<QueryResult, String> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.user,
        config.password.unwrap_or_default(),
        config.host,
        config.port,
        config.database
    );
    let pool = MySqlPool::connect(&url).await.map_err(|e| e.to_string())?;

    let rows = sqlx::query(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
        }); // Handle empty result set
    }
    let columns = rows
        .first()
        .unwrap()
        .columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    let mut result_rows = Vec::new();
    for row in rows {
        let mut db_row = DbRow(Vec::new());
        for i in 0..row.columns().len() {
            // Attempt to get the value as a String, handling different types
            let value: String = match row.try_get::<String, _>(i) {
                Ok(v) => v,
                Err(_) => {
                    match row.try_get::<i64, _>(i) {
                        Ok(v) => v.to_string(),
                        Err(_) => match row.try_get::<f64, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => match row.try_get::<bool, _>(i) {
                                Ok(v) => v.to_string(),
                                Err(_) => match row.try_get::<Vec<u8>, _>(i) {
                                    //for blobs
                                    Ok(v) => format!("{:?}", v),
                                    Err(_) => "unsupported".to_string(), //if we cant get it at all
                                },
                            },
                        },
                    }
                }
            };
            db_row.0.push(value);
        }
        result_rows.push(db_row);
    }

    Ok(QueryResult {
        columns,
        rows: result_rows,
    })
}

async fn execute_postgres_query(
    config: ConnectionConfig,
    query: String,
) -> Result<QueryResult, String> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password.unwrap_or_default(),
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(&url).await.map_err(|e| e.to_string())?;
    let rows = sqlx::query(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
        });
    }

    let columns = rows
        .first()
        .unwrap()
        .columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    let mut result_rows = Vec::new();
    for row in rows {
        let mut db_row = DbRow(Vec::new());

        for i in 0..row.columns().len() {
            let value: String = match row.try_get::<String, _>(i) {
                Ok(v) => v,
                Err(_) => {
                    match row.try_get::<i64, _>(i) {
                        Ok(v) => v.to_string(),
                        Err(_) => match row.try_get::<f64, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => match row.try_get::<bool, _>(i) {
                                Ok(v) => v.to_string(),
                                Err(_) => match row.try_get::<Vec<u8>, _>(i) {
                                    //for blobs
                                    Ok(v) => format!("{:?}", v),
                                    Err(_) => "unsupported".to_string(), //if we cant get it at all
                                },
                            },
                        },
                    }
                }
            };
            db_row.0.push(value);
        }
        result_rows.push(db_row);
    }
    Ok(QueryResult {
        columns,
        rows: result_rows,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect_to_db, execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
