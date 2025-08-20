use std::time::{UNIX_EPOCH, SystemTime};
use anyhow::Result;
use rusqlite::{params, Connection};

pub fn insert_schedule(db_connection: &Connection, name: &str, schedule: &[u8], hash: u64) -> Result<()>{
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards lol. Your BIOS has some serious issues.")
        .as_secs();

    db_connection.execute("
        INSERT INTO schedules(name, content, hash, timestamp) VALUES (?1, ?2, ?3, ?4)
    ", params![name, schedule, hash.to_be_bytes().as_slice(), now])?;

    Ok(())
}
