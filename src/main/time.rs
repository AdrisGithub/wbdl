use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::WBDLError;
use crate::month::Month;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Debug)]
pub struct Hour(u8);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Debug)]
pub struct Minute(u8);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Debug)]
pub struct Second(u8);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Debug)]
pub struct Day(u8);

impl Hour {
    pub const MIN: Hour = Hour(0);
    pub const MAX: Hour = Hour(23);
    pub fn next(&self) -> Hour {
        if self == &Hour::MAX {
            Hour::MIN
        } else {
            Hour(self.0 + 1)
        }
    }

    pub fn previous(&self) -> Hour {
        if self == &Hour::MIN {
            Hour::MAX
        } else {
            Hour(self.0 - 1)
        }
    }
}

impl Day {
    pub const MIN: Day = Day(0);
    pub fn max(year: u16, month: Month) -> Day {
        Self(Day::get_days_per_month(year)[month.ordinal()])
    }
    pub const fn get_days_per_month(year: u16) -> [u8; 13] {
        [
            0,
            31,
            if crate::util::is_leap_year(year) { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ]
    }
    pub fn next(&self, year: u16, month: Month) -> Day {
        let info = Day::get_days_per_month(year)[month.ordinal()];
        if self.0 == info {
            Day::MIN
        } else {
            Day(self.0 + 1)
        }
    }
    pub fn previous(&self, year: u16, month: Month) -> Day {
        if self == &Day::MIN {
            let info = Day::get_days_per_month(year)[month.previous().ordinal()];
            Self(info)
        } else {
            Day(self.0 - 1)
        }
    }
}

impl Minute {
    pub const MIN: Minute = Minute(0);
    pub const MAX: Minute = Minute(59);
    pub fn next(&self) -> Minute {
        if self == &Minute::MAX {
            Minute::MIN
        } else {
            Minute(self.0 + 1)
        }
    }

    pub fn previous(&self) -> Minute {
        if self == &Minute::MIN {
            Minute::MAX
        } else {
            Minute(self.0 - 1)
        }
    }
}

impl Second {
    pub const MIN: Second = Second(0);
    pub const MAX: Second = Second(59);

    pub fn next(&self) -> Second {
        if self == &Second::MAX {
            Second::MIN
        } else {
            Second(self.0 + 1)
        }
    }

    pub fn previous(&self) -> Second {
        if self == &Second::MIN {
            Second::MAX
        } else {
            Second(self.0 - 1)
        }
    }
}


impl Display for Hour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Display for Minute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Display for Second {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl TryFrom<&str> for Hour {
    type Error = WBDLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Hour::try_from(u8::from_str(value).map_err(|_err| WBDLError)?)
    }
}

impl TryFrom<(&str, u16, Month)> for Day {
    type Error = WBDLError;
    fn try_from(value: (&str, u16, Month)) -> Result<Self, Self::Error> {
        Ok(Day::try_from((u8::from_str(value.0).map_err(|_err| WBDLError)?, value.1, value.2)).map_err(|_err| WBDLError)?)
    }
}

impl TryFrom<&str> for Minute {
    type Error = WBDLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Minute::try_from(u8::from_str(value).map_err(|_err| WBDLError)?)
    }
}

impl TryFrom<&str> for Second {
    type Error = WBDLError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Second::try_from(u8::from_str(value).map_err(|_err| WBDLError)?)
    }
}

impl TryFrom<u8> for Hour {
    type Error = WBDLError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 24 {
            Ok(Self(value))
        } else {
            Err(WBDLError)
        }
    }
}

impl TryFrom<(u8, u16, Month)> for Day {
    type Error = WBDLError;
    fn try_from(value: (u8, u16, Month)) -> Result<Self, Self::Error> {
        if value.0 <= Day::get_days_per_month(value.1)[value.2.ordinal()] {
            Ok(Self(value.0))
        } else {
            Err(WBDLError)
        }
    }
}

impl TryFrom<u8> for Minute {
    type Error = WBDLError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 60 {
            Ok(Self(value))
        } else {
            Err(WBDLError)
        }
    }
}

impl TryFrom<u8> for Second {
    type Error = WBDLError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 60 {
            Ok(Self(value))
        } else {
            Err(WBDLError)
        }
    }
}