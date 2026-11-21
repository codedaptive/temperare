use std::time::{SystemTime, UNIX_EPOCH};

pub fn utc_date(time: SystemTime) -> String {
    let mut days = time.duration_since(UNIX_EPOCH).unwrap().as_secs() / 86_400;
    let mut year = 1970;
    loop {
        let year_days = if leap_year(year) { 366 } else { 365 };
        if days < year_days {
            break;
        }
        days -= year_days;
        year += 1;
    }
    let months = [31, if leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (month, length) in months.iter().enumerate() {
        if days < *length {
            return format!("{year:04}-{:02}-{:02}", month + 1, days + 1);
        }
        days -= length;
    }
    unreachable!()
}

fn leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
