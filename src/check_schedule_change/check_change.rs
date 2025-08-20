use anyhow::Result;
use rusqlite::{Connection, params};

use crate::check_schedule_change::hash_schedule;

fn get_last_schedule_hash(db_connection: &Connection) -> Result<Option<u64>> {
    let query = "SELECT hash FROM schedules ORDER BY timestamp DESC LIMIT 1";

    let mut statement = db_connection.prepare(query)?;

    let hash_bytes: Option<Vec<u8>> = statement.query_row(
        params![],
        |row| row.get(0),
    ).ok();

    if hash_bytes.is_some() {
        let hash_slice: [u8; 8] = hash_bytes.unwrap().as_slice().try_into()?;
        let hash = u64::from_be_bytes(hash_slice);
        return Ok(Some(hash));
    }

    Ok(None)
}

pub fn has_changed(db_connection: &Connection, current_schedule: &[u8]) -> Result<bool> {
    let last_schedule_hash = get_last_schedule_hash(db_connection)?;

    Ok(last_schedule_hash.is_none_or(|last_hash| {
        last_hash != hash_schedule(current_schedule)
    }))
}
