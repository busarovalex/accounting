use chrono::prelude::*;

pub fn last_day_of_month(date: NaiveDate) -> NaiveDate {
    let year = date.year();
    let month = date.month();
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
}

pub fn start_of_day() -> NaiveTime {
    NaiveTime::from_hms(0, 0, 0)
}

pub fn end_of_day() -> NaiveTime {
    NaiveTime::from_hms(23, 59, 59)
}
