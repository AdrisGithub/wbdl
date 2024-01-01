use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::time::SystemTime;

use crate::error::WBDLError;
use crate::month::Month;
use crate::time::{Day, Hour, Minute, Second};
use crate::util::{get_date_time, EPOCH_YEAR};

#[derive(Eq, Copy, Clone, PartialEq,Hash)]
pub struct Date {
    day: Day,
    month: Month,
    year: u16,
    hour: Hour,
    minute: Minute,
    second: Second,
}
impl Default for Date{
    fn default() -> Self {
        Date::UNIX_EPOCH
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            None
        }
    }
    #[allow(clippy::comparison_chain)]
    fn lt(&self, other: &Self) -> bool {
        if self.year < other.year {
            return true;
        } else if self.year > other.year {
            return false;
        }
        if self.month < other.month {
            return true;
        } else if self.month > other.month {
            return false;
        }
        if self.day < other.day {
            return true;
        } else if self.day > other.day {
            return false;
        }
        if self.hour < other.hour {
            return true;
        } else if self.hour > other.hour {
            return false;
        }
        if self.minute < other.minute {
            return true;
        } else if self.minute > other.minute {
            return false;
        }
        self.second < other.second
    }
    fn le(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            self < other
        }
    }
    fn gt(&self, other: &Self) -> bool {
        !self.le(other)
    }

    fn ge(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            self > other
        }
    }
}

impl Date {
    pub const UNIX_EPOCH: Date = Date {
        month: Month::MIN,
        year: EPOCH_YEAR,
        day: Day::MIN,
        hour: Hour::MIN,
        second: Second::MIN,
        minute: Minute::MIN,
    };
    pub fn now_unchecked() -> Date {
        Date::now().unwrap()
    }
    pub fn now() -> Result<Date, WBDLError> {
        Self::try_from(
            SystemTime::UNIX_EPOCH
                .elapsed()
                .map(|va| va.as_secs())
                .map_err(|_err| WBDLError)?,
        )
    }
    pub fn add_min(mut self) -> Self {
        if self.minute >= Minute::MAX {
            self.add_hour();
        }
        self.minute = self.minute.next();
        self
    }
    pub fn add_hour(mut self) -> Self{
        if self.hour >= Hour::MAX {
            self.add_day();
        }
        self.hour = self.hour.next();
        self
    }
    pub fn add_second(mut self) -> Self {
        if self.second >= Second::MAX {
            self.add_min();
        }
        self.second = self.second.next();
        self
    }
    pub fn add_day(mut self) -> Self {
        let old = (self.year, self.month);
        if self.day >= Day::max(self.year, self.month) {
            self.add_month();
        }
        self.day = self.day.next(old.0, old.1);
        self
    }
    pub fn add_month(mut self) -> Self {
        if self.month.eq(&Month::December) {
            self.add_year();
        }
        self.month = self.month.next();
        self
    }
    pub const fn add_year(mut self) -> Self {
        self.year += 1;
        self
    }
    pub fn next_minute(self) -> Self {
        self.add_min();
        self.reset_until_seconds()
    }
    pub fn next_hour(self) -> Self{
        self.add_hour();
        self.reset_until_minutes()
    }
    pub fn next_day(self) -> Self {
        self.add_day();
        self.reset_until_hours()
    }
    pub fn next_month(self) -> Self {
        self.add_month();
        self.reset_until_days()
    }
    pub fn next_year(self) -> Self {
        self.add_year();
        self.reset_until_months()
    }
    pub const fn reset_until_seconds(mut self) -> Self {
        self.second = Second::MIN;
        self
    }
    pub const fn reset_until_minutes(mut self) -> Self {
        self.second = Second::MIN;
        self.reset_until_seconds()
    }
    pub const fn reset_until_hours(mut self) -> Self {
        self.hour = Hour::MIN;
        self.reset_until_minutes()
    }
    pub const fn reset_until_days(mut self) -> Self {
        self.day = Day::MIN;
        self.reset_until_hours()
    }
    pub const fn reset_until_months(mut self) -> Self {
        self.month = Month::MIN;
        self.reset_until_days()
    }
    pub const fn reset_until_years(mut self) -> Self {
        self.year = EPOCH_YEAR;
        self.reset_until_months()
    }
}

impl TryFrom<u64> for Date {
    type Error = WBDLError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let pieces = get_date_time(value);
        let month = Month::try_from(pieces[1] as usize)?;
        Ok(Self {
            year: pieces[0],
            month,
            day: Day::try_from((pieces[2] as u8, pieces[0], month))?,
            hour: Hour::try_from(pieces[3] as u8)?,
            minute: Minute::try_from(pieces[4] as u8)?,
            second: Second::try_from(pieces[5] as u8)?,
        })
    }
}

impl TryFrom<&str> for Date {
    type Error = WBDLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split('T');
        let mut date = split.next().ok_or(WBDLError)?.split('-');
        let mut time = split.next().ok_or(WBDLError)?.split(':');
        let year = date
            .next()
            .map(u16::from_str)
            .ok_or(WBDLError)?
            .map_err(|_err| WBDLError)?;
        let month = Month::try_from(date.next().ok_or(WBDLError)?)?;
        Ok(Self {
            year,
            month,
            day: Day::try_from((date.next().ok_or(WBDLError)?, year, month))?,
            hour: Hour::try_from(time.next().ok_or(WBDLError)?)?,
            minute: Minute::try_from(time.next().ok_or(WBDLError)?)?,
            second: Second::try_from(time.next().ok_or(WBDLError)?)?,
        })
    }
}

impl TryFrom<SystemTime> for Date {
    type Error = WBDLError;
    fn try_from(value: SystemTime) -> Result<Self, Self::Error> {
        value
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|val| val.as_secs())
            .map(Date::try_from)
            .map_err(|_err| WBDLError)?
    }
}

impl Iterator for Date {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.add_second();
        Some(*self)
    }
}

impl TryFrom<String> for Date {
    type Error = WBDLError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //"2004-06-14T23:34:30"
        write!(
            f,
            "{}-{}-{}T{}:{}:{}",
            self.year,
            self.month.ordinal(),
            self.day,
            self.hour,
            self.minute,
            self.second
        )
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
mod tests {
    use std::time::SystemTime;

    use crate::date::Date;

    #[test]
    pub fn date_now() {
        let now = Date::now();
        print!("{:?}", now.unwrap());
    }

    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    pub fn mapping() {
        let pre = Some(Date::UNIX_EPOCH);
        let post = pre.map(|val| val.to_string()).map(Date::try_from).unwrap();
        assert_eq!(pre.unwrap(), post.unwrap())
    }

    #[test]
    pub fn equals() {
        let first = Date::UNIX_EPOCH;
        let second = Date::UNIX_EPOCH;
        assert_eq!(first, second);
        assert!(!(first > second));
        assert!(!(first < second));
    }

    #[test]
    pub fn not_equals() {
        let first = Date::UNIX_EPOCH;
        let second = Date::now().unwrap();
        assert_ne!(first, second);
        assert!(!(first > second));
        assert!(first < second);
    }

    #[test]
    pub fn bigger() {
        let first = Date::UNIX_EPOCH;
        let second = Date::UNIX_EPOCH.add_second();
        assert_ne!(first, second);
        assert!(first < second);
        assert!(!(first > second));
    }

    #[test]
    pub fn smaller() {
        let first = Date::UNIX_EPOCH.add_second();
        let second = Date::UNIX_EPOCH;
        assert_ne!(first, second);
        assert!(!(first < second));
        assert!(first > second);
    }

    #[test]
    pub fn unix_epoch() {
        let first = Date::UNIX_EPOCH;
        let second = Date::try_from(SystemTime::UNIX_EPOCH).unwrap();
        assert_eq!(first, second)
    }
}
