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

#[cfg(test)]
mod test {
    use crate::util;

    #[test]
    pub fn is_leap_year() {
        for leap_year in get_leap_years() {
            assert!(util::is_leap_year(leap_year as u16))
        }
    }

    #[test]
    pub fn is_not_leap_year() {
        for leap_year in get_leap_years() {
            assert!(!util::is_leap_year((leap_year + 1) as u16))
        }
    }

    pub fn get_leap_years() -> Vec<i32> {
        vec![
            1804, 1808, 1812, 1816, 1820, 1824, 1828, 1832, 1836, 1840, 1844, 1848, 1852, 1856,
            1860, 1864, 1868, 1872, 1876, 1880, 1884, 1888, 1892, 1896, 1904, 1908, 1912, 1916,
            1920, 1924, 1928, 1932, 1936, 1940, 1944, 1948, 1952, 1956, 1960, 1964, 1968, 1972,
            1976, 1980, 1984, 1988, 1992, 1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024, 2028,
            2032, 2036, 2040, 2044, 2048, 2052, 2056, 2060, 2064, 2068, 2072, 2076, 2080, 2084,
            2088, 2092, 2096, 2104, 2108, 2112, 2116, 2120, 2124, 2128, 2132, 2136, 2140, 2144,
            2148, 2152, 2156, 2160, 2164, 2168, 2172, 2176, 2180, 2184, 2188, 2192, 2196, 2204,
            2208, 2212, 2216, 2220, 2224, 2228, 2232, 2236, 2240, 2244, 2248, 2252, 2256, 2260,
            2264, 2268, 2272, 2276, 2280, 2284, 2288, 2292, 2296, 2304, 2308, 2312, 2316, 2320,
            2324, 2328, 2332, 2336, 2340, 2344, 2348, 2352, 2356, 2360, 2364, 2368, 2372, 2376,
            2380, 2384, 2388, 2392, 2396, 2400,
        ]
    }
}
