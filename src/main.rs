mod get_schedule;
mod check_schedule_change;
mod schedule_storage;

use anyhow::Result;

use crate::schedule_storage::insert_schedule;
use std::process::Command;

fn notify_change() {
    let _ = Command::new("notify-send")
        .args(["TIB schedule has changed."])
        .spawn().unwrap().wait();
}

fn main() -> Result<()> {
    let (current_schedule, schedule_url) = get_schedule::get_schedule_pdf()?;
    let db_connection = schedule_storage::get_connection()?;

    if check_schedule_change::has_changed(&db_connection, current_schedule.as_slice())? {
        let schedule_hash = check_schedule_change::hash_schedule(&current_schedule);

        insert_schedule(&db_connection, &schedule_url, &current_schedule, schedule_hash)?;

        notify_change();
    } else {
        println!("Same old schedule than last time.");
    }

    Ok(())
}
