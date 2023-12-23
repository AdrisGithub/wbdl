use crate::time::Day;

const SECONDS_PER_MINUTE: u8 = 60u8;
const SECONDS_PER_HOUR: u16 = SECONDS_PER_MINUTE as u16 * SECONDS_PER_MINUTE as u16;
const SECONDS_PER_DAY: u32 = 24u32 * SECONDS_PER_HOUR as u32;
const DAYS_PER_YEAR: u16 = 365;
const DAYS_PER_LEAP_YEAR: u16 = DAYS_PER_YEAR + 1;
const EPOCH_MONTH: u8 = 1;
pub(crate) const EPOCH_YEAR: u16 = 1970;

pub const fn is_leap_year(year: u16) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

pub const fn get_days_for_year(year: u16) -> u16 {
    if is_leap_year(year) {
        DAYS_PER_LEAP_YEAR
    } else {
        DAYS_PER_YEAR
    }
}

pub const fn get_date_time(timestamp: u64) -> [u16; 6] {
    let mut days = timestamp / SECONDS_PER_DAY as u64;
    let mut year = EPOCH_YEAR;
    while days >= get_days_for_year(year) as u64 {
        days -= get_days_for_year(year) as u64;
        year += 1;
    }
    let days_per_month = Day::get_days_per_month(year);
    let mut month = EPOCH_MONTH;
    while days >= days_per_month[month as usize] as u64 {
        days -= days_per_month[month as usize] as u64;
        month += 1;
    }
    let day = days + 1;
    let seconds_remaining = timestamp % SECONDS_PER_DAY as u64;
    let hour = seconds_remaining / SECONDS_PER_HOUR as u64;
    let minute = (seconds_remaining / SECONDS_PER_MINUTE as u64) % SECONDS_PER_MINUTE as u64;
    let second = seconds_remaining % SECONDS_PER_MINUTE as u64;
    [
        year,
        month as u16,
        day as u16,
        hour as u16,
        minute as u16,
        second as u16,
    ]
}
