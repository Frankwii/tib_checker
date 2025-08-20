mod get_schedule;
mod check_schedule_change;
mod persist_schedule;

fn main() {
    let stdout = match get_schedule::get_schedule_pdf() {
        Ok(s) => {String::from_utf8(s).unwrap()},
        Err(e) => {e.to_string()}
    };
    println!("{stdout}");
}
