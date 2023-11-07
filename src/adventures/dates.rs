use chrono::{Utc, Duration};
use rand::Rng;

const DEADLINE_RANGE: std::ops::Range<i64> = 20..60;

pub fn get_random_date(start: String,end: String) -> chrono::DateTime<Utc> {
    mockd::datetime::date_range(start, end)
}

pub fn get_random_deadline(date: chrono::DateTime<Utc>, rng: &mut rand::rngs::ThreadRng) -> chrono::DateTime<Utc> {
    let offset = rng.gen_range(DEADLINE_RANGE);
    date + Duration::days(offset)
}

pub fn date_to_string(date: chrono::DateTime<Utc>) -> String {
    date.to_string()[0..10].to_string()
}
