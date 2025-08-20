// Get a connection to a sqlite database or create one if it doesn't exist.

use std::env;
use std::path::PathBuf;

use rusqlite::{params, Connection};

pub enum DbConnectionError {
    Path(String),
    CreateDir(String),
    OpenConnection(String),
}

impl From<DbConnectionError> for anyhow::Error {
    fn from(value: DbConnectionError) -> Self {
        Self::msg(
            match value {
                DbConnectionError::Path(e) => e,
                DbConnectionError::CreateDir(e) => e,
                DbConnectionError::OpenConnection(e) => e
            }
        )
    }
}

fn data_dir() -> Result<PathBuf, DbConnectionError> {
    // XDG_DATA_HOME or fallback to $HOME/.local/share (Linux/BSD convention)
    if let Some(xdg) = env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg));
    }
    let res = env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share"));

    res.ok_or(DbConnectionError::Path(
        "Unable to find home directory. Is the $HOME envvar set?".into(),
    ))
}

/// "Home" directory for this program; not the user's
fn get_home_directory() -> Result<PathBuf, DbConnectionError> {
    let data_dir = data_dir()?.join("tib_checker");

    if !data_dir.exists() {
        match std::fs::create_dir_all(data_dir.clone()) {
            Ok(()) => {}
            Err(e) => return Err(DbConnectionError::CreateDir(e.to_string())),
        }
    }

    Ok(data_dir)
}

fn ensure_schedule_table_exists(db_connection: &Connection) -> Result<(), DbConnectionError> {
    let query = "
        CREATE TABLE schedules (
            name TEXT,
            content BLOB NOT NULL,
            hash BLOB NOT NULL,
            timestamp INTEGER NOT NULL
        )
    ";
    // TODO: Properly check whether the table exists and return an appropriate
    // error if the problem was found creating it.
    let _ = db_connection.execute(query, params![]); 
    Ok(())
}

pub fn get_connection() -> Result<Connection, DbConnectionError> {
    let sqlite_db_file = get_home_directory()?.join("database.db");

    let connection = Connection::open(sqlite_db_file)
        .map_err(|e| DbConnectionError::OpenConnection(e.to_string()))?;

    ensure_schedule_table_exists(&connection)?;
    Ok(connection)
}
