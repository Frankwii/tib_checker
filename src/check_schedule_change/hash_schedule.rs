use std::hash::{DefaultHasher, Hasher};

pub fn hash_schedule(schedule_pdf: &[u8]) -> u64 {
    let mut h = DefaultHasher::new();
    h.write(schedule_pdf);
    h.finish()
}
