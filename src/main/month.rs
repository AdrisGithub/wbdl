use std::str::FromStr;

use crate::error::WBDLError;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Season {
    Spring = 1,
    Summer,
    Autumn,
    Winter,
}

impl Month {
    pub const MIN: Month = Month::January;
    pub const MAX: Month = Month::December;
    pub fn get_season(&self) -> Season {
        Season::from(self)
    }
    pub fn ordinal(&self) -> usize {
        self.clone() as usize
    }
    pub const fn next(&self) -> Month {
        return match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March =>  Month::April,
            Month::April =>  Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July =>  Month::August,
            Month::August =>  Month::September,
            Month::September => Month::October,
            Month::October =>  Month::November,
            Month::November => Month::December,
            Month::December => Month::January
        }
    }
    pub fn previous(&self) -> Month {
        return match self {
            Month::January => Month::December,
            Month::February => Month::January,
            Month::March => Month::February,
            Month::April => Month::March,
            Month::May => Month::April,
            Month::June => Month::May,
            Month::July => Month::June,
            Month::August => Month::July,
            Month::September => Month::August,
            Month::October => Month::September,
            Month::November => Month::October,
            Month::December => Month::November
        }
    }
}

impl Season {
    pub fn ordinal(&self) -> usize {
        self.clone() as usize
    }
    pub const MIN: Season = Season::Spring;
    pub const MAX: Season = Season::Winter;
}

impl From<&Month> for Season {
    fn from(value: &Month) -> Self {
        match value {
            Month::January => Season::Winter,
            Month::February => Season::Spring,
            Month::March => Season::Spring,
            Month::April => Season::Spring,
            Month::May => Season::Summer,
            Month::June => Season::Summer,
            Month::July => Season::Summer,
            Month::August => Season::Autumn,
            Month::September => Season::Autumn,
            Month::October => Season::Summer,
            Month::November => Season::Winter,
            Month::December => Season::Winter,
        }
    }
}

impl TryFrom<usize> for Season {
    type Error = WBDLError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Season::Spring),
            2 => Ok(Season::Summer),
            3 => Ok(Season::Autumn),
            4 => Ok(Season::Winter),
            _ => Err(WBDLError)
        }
    }
}

impl TryFrom<usize> for Month {
    type Error = WBDLError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(WBDLError)
        }
    }
}

impl TryFrom<&str> for Month {
    type Error = WBDLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Month::try_from(usize::from_str(value).map_err(|_err| WBDLError)?)
    }
}