use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::time::SystemTime;

use crate::error::WBDLError;
use crate::month::Month;
use crate::time::{Day, Hour, Minute, Second};
use crate::util::{EPOCH_YEAR, get_date_time};

#[derive(Eq, Copy, Clone, PartialEq)]
pub struct Date {
    day: Day,
    month: Month,
    year: u16,
    hour: Hour,
    minute: Minute,
    second: Second,
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
    pub fn now_unchecked() -> Date {
        Date::now().unwrap()
    }
    pub fn now() -> Result<Date, WBDLError> {
        Self::try_from(SystemTime::UNIX_EPOCH.elapsed().map(|va| va.as_secs()).map_err(|_err| WBDLError)?)
    }
    pub fn unix_epoch() -> Date {
        Date::try_from(0).unwrap()
    }
    pub fn add_min(&mut self) {
        if self.minute == Minute::MAX {
            self.add_hour();
        }
        self.minute = self.minute.next();
    }
    pub fn add_hour(&mut self) {
        if self.hour == Hour::MAX {
            self.add_day();
        }
        self.hour = self.hour.next();
    }
    pub fn add_second(&mut self) {
        if self.second == Second::MAX {
            self.add_min();
        }
        self.second = self.second.next();
    }
    pub fn add_day(&mut self) {
        if self.day == Day::max(self.year, self.month) {
            self.add_month();
        }
        self.day = self.day.next(self.year, self.month);
    }
    pub fn add_month(&mut self) {
        if self.month.eq(&Month::December) {
            self.add_year();
        }
        self.month = self.month.next();
    }
    pub fn add_year(&mut self) {
        self.year += 1;
    }
    pub fn next_minute(&mut self) {
        self.add_min();
        self.reset_until_seconds();
    }
    pub fn next_hour(&mut self) {
        self.add_hour();
        self.reset_until_minutes();
    }
    pub fn next_day(&mut self) {
        self.add_day();
        self.reset_until_hours();
    }
    pub fn next_month(&mut self) {
        self.add_month();
        self.reset_until_days();
    }
    pub fn next_year(&mut self) {
        self.add_year();
        self.reset_until_months();
    }
    pub fn reset_until_seconds(&mut self){
        self.second = Second::MIN;
    }
    pub fn reset_until_minutes(&mut self){
        self.reset_until_seconds();
        self.second = Second::MIN;
    }
    pub fn reset_until_hours(&mut self){
        self.reset_until_minutes();
        self.hour = Hour::MIN;
    }
    pub fn reset_until_days(&mut self){
        self.reset_until_hours();
        self.day = Day::MIN;
    }
    pub fn reset_until_months(&mut self){
        self.reset_until_days();
        self.month = Month::MIN;
    }
    pub fn reset_until_years(&mut self){
        self.reset_until_months();
        self.year = EPOCH_YEAR;
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

impl TryFrom<String> for Date {
    type Error = WBDLError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split('T');
        let mut date = split.next().ok_or(WBDLError)?.split('-');
        let mut time = split.next().ok_or(WBDLError)?.split(':');
        let year = date.next().map(u16::from_str).ok_or(WBDLError)?.map_err(|_err| WBDLError)?;
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

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //"2004-06-14T23:34:30"
        write!(f, "{}-{}-{}T{}:{}:{}", self.year, self.month.ordinal(), self.day, self.hour, self.minute, self.second)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;

    #[test]
    pub fn date_now() {
        let now = Date::now();
        let now = now.map(|val| val.to_string()).map(Date::try_from);
        println!("{:?} {:?}", now, Date::unix_epoch());
        println!("{}", Date::now().unwrap() == Date::unix_epoch())
    }
}



