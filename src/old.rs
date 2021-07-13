use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc, Weekday};
// use std::convert::TryFrom;
use std::fmt::{self, Display};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Seasonal {
    pub year: i32,
    pub season: Season,
    pub week: u8,
    pub day: Weekday,
}

impl Seasonal {
    pub fn to_date(self) -> NaiveDate {
        let season_start = dates(self.year)[self.season as usize];
        let week = season_start.iso_week().week() + u32::from(self.week) - 1;
        NaiveDate::from_isoywd(self.year, week, self.day)
    }

    // pub fn from_date<Tz: TimeZone>(date: Date<Tz>) -> Seasonal {
    //     let iso = date.iso_week();
    //     let year = iso.year();
    //     let [march, june, sept, dec] = dates(year);
    //     let march_week = march.with_timezone(date.timezone()).iso_week().week();
    //     let june_week = june.with_timezone(date.timezone()).iso_week().week();
    //     let sept_week = sept.with_timezone(date.timezone()).iso_week().week();
    //     let dec_week = dec.with_timezone(date.timezone()).iso_week().week();

    //     let (season, week) = if iso < march_week {
    //         panic!()
    //     } else if iso < june_week {
    //         (Season::Spring, iso.week() - march_week + 1)
    //     } else if iso < sept_week {
    //         (Season::Summer, iso.week() - june_week + 1)
    //     } else if iso < dec_week {
    //         (Season::Autumn, iso.week() - sept_week + 1)
    //     } else {
    //         panic!()
    //     };

    //     Seasonal {
    //         year,
    //         season,
    //         week: u8::try_from(week).unwrap(),
    //         day: date.weekday(),
    //     }
    // }
}

impl Display for Seasonal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} w{} {}",
            self.year, self.season, self.week, self.day
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Season {
    Spring = 0,
    Summer = 1,
    Autumn = 2,
    Winter = 3,
}

impl Season {
    pub fn flip_hemisphere(self) -> Season {
        use Season::*;
        match self {
            Spring => Autumn,
            Summer => Winter,
            Autumn => Spring,
            Winter => Summer,
        }
    }

    // pub fn start_date(self) ->
}

// TODO: Compute these instead
pub fn dates(year: i32) -> [DateTime<Utc>; 4] {
    match year {
        2001 => [
            Utc.ymd(2001, 3, 20).and_hms(13, 31, 0),
            Utc.ymd(2001, 6, 21).and_hms(07, 38, 0),
            Utc.ymd(2001, 9, 22).and_hms(23, 05, 0),
            Utc.ymd(2001, 12, 21).and_hms(19, 22, 0),
        ],
        2002 => [
            Utc.ymd(2002, 3, 20).and_hms(19, 16, 0),
            Utc.ymd(2002, 6, 21).and_hms(13, 25, 0),
            Utc.ymd(2002, 9, 23).and_hms(04, 56, 0),
            Utc.ymd(2002, 12, 22).and_hms(01, 15, 0),
        ],
        2003 => [
            Utc.ymd(2003, 3, 21).and_hms(01, 00, 0),
            Utc.ymd(2003, 6, 21).and_hms(19, 11, 0),
            Utc.ymd(2003, 9, 23).and_hms(10, 47, 0),
            Utc.ymd(2003, 12, 22).and_hms(07, 04, 0),
        ],
        2004 => [
            Utc.ymd(2004, 3, 20).and_hms(06, 49, 0),
            Utc.ymd(2004, 6, 21).and_hms(00, 57, 0),
            Utc.ymd(2004, 9, 22).and_hms(16, 30, 0),
            Utc.ymd(2004, 12, 21).and_hms(12, 42, 0),
        ],
        2005 => [
            Utc.ymd(2005, 3, 20).and_hms(12, 34, 0),
            Utc.ymd(2005, 6, 21).and_hms(06, 46, 0),
            Utc.ymd(2005, 9, 22).and_hms(22, 23, 0),
            Utc.ymd(2005, 12, 21).and_hms(18, 35, 0),
        ],
        2006 => [
            Utc.ymd(2006, 3, 20).and_hms(18, 25, 0),
            Utc.ymd(2006, 6, 21).and_hms(12, 26, 0),
            Utc.ymd(2006, 9, 23).and_hms(04, 04, 0),
            Utc.ymd(2006, 12, 22).and_hms(00, 22, 0),
        ],
        2007 => [
            Utc.ymd(2007, 3, 21).and_hms(00, 07, 0),
            Utc.ymd(2007, 6, 21).and_hms(18, 06, 0),
            Utc.ymd(2007, 9, 23).and_hms(09, 51, 0),
            Utc.ymd(2007, 12, 22).and_hms(06, 08, 0),
        ],
        2008 => [
            Utc.ymd(2008, 3, 20).and_hms(05, 49, 0),
            Utc.ymd(2008, 6, 21).and_hms(00, 00, 0),
            Utc.ymd(2008, 9, 22).and_hms(15, 45, 0),
            Utc.ymd(2008, 12, 21).and_hms(12, 04, 0),
        ],
        2009 => [
            Utc.ymd(2009, 3, 20).and_hms(11, 44, 0),
            Utc.ymd(2009, 6, 21).and_hms(05, 45, 0),
            Utc.ymd(2009, 9, 22).and_hms(21, 18, 0),
            Utc.ymd(2009, 12, 21).and_hms(17, 47, 0),
        ],
        2010 => [
            Utc.ymd(2010, 3, 20).and_hms(17, 32, 0),
            Utc.ymd(2010, 6, 21).and_hms(11, 28, 0),
            Utc.ymd(2010, 9, 23).and_hms(03, 09, 0),
            Utc.ymd(2010, 12, 21).and_hms(23, 38, 0),
        ],
        2011 => [
            Utc.ymd(2011, 3, 20).and_hms(23, 21, 0),
            Utc.ymd(2011, 6, 21).and_hms(17, 16, 0),
            Utc.ymd(2011, 9, 23).and_hms(09, 05, 0),
            Utc.ymd(2011, 12, 22).and_hms(05, 30, 0),
        ],
        2012 => [
            Utc.ymd(2012, 3, 20).and_hms(05, 15, 0),
            Utc.ymd(2012, 6, 20).and_hms(23, 08, 0),
            Utc.ymd(2012, 9, 22).and_hms(14, 49, 0),
            Utc.ymd(2012, 12, 21).and_hms(11, 12, 0),
        ],
        2013 => [
            Utc.ymd(2013, 3, 20).and_hms(11, 02, 0),
            Utc.ymd(2013, 6, 21).and_hms(05, 04, 0),
            Utc.ymd(2013, 9, 22).and_hms(20, 44, 0),
            Utc.ymd(2013, 12, 21).and_hms(17, 11, 0),
        ],
        2014 => [
            Utc.ymd(2014, 3, 20).and_hms(16, 57, 0),
            Utc.ymd(2014, 6, 21).and_hms(10, 52, 0),
            Utc.ymd(2014, 9, 23).and_hms(02, 30, 0),
            Utc.ymd(2014, 12, 21).and_hms(23, 03, 0),
        ],
        2015 => [
            Utc.ymd(2015, 3, 20).and_hms(22, 45, 0),
            Utc.ymd(2015, 6, 21).and_hms(16, 38, 0),
            Utc.ymd(2015, 9, 23).and_hms(08, 20, 0),
            Utc.ymd(2015, 12, 22).and_hms(04, 48, 0),
        ],
        2016 => [
            Utc.ymd(2016, 3, 20).and_hms(04, 31, 0),
            Utc.ymd(2016, 6, 20).and_hms(22, 35, 0),
            Utc.ymd(2016, 9, 22).and_hms(14, 21, 0),
            Utc.ymd(2016, 12, 21).and_hms(10, 45, 0),
        ],
        2017 => [
            Utc.ymd(2017, 3, 20).and_hms(10, 29, 0),
            Utc.ymd(2017, 6, 21).and_hms(04, 25, 0),
            Utc.ymd(2017, 9, 22).and_hms(20, 02, 0),
            Utc.ymd(2017, 12, 21).and_hms(16, 29, 0),
        ],
        2018 => [
            Utc.ymd(2018, 3, 20).and_hms(16, 15, 0),
            Utc.ymd(2018, 6, 21).and_hms(10, 07, 0),
            Utc.ymd(2018, 9, 23).and_hms(01, 54, 0),
            Utc.ymd(2018, 12, 21).and_hms(22, 22, 0),
        ],
        2019 => [
            Utc.ymd(2019, 3, 20).and_hms(21, 58, 0),
            Utc.ymd(2019, 6, 21).and_hms(15, 54, 0),
            Utc.ymd(2019, 9, 23).and_hms(07, 50, 0),
            Utc.ymd(2019, 12, 22).and_hms(04, 19, 0),
        ],
        2020 => [
            Utc.ymd(2020, 3, 20).and_hms(03, 50, 0),
            Utc.ymd(2020, 6, 20).and_hms(21, 43, 0),
            Utc.ymd(2020, 9, 22).and_hms(13, 31, 0),
            Utc.ymd(2020, 12, 21).and_hms(10, 03, 0),
        ],
        2021 => [
            Utc.ymd(2021, 3, 20).and_hms(09, 37, 0),
            Utc.ymd(2021, 6, 21).and_hms(03, 32, 0),
            Utc.ymd(2021, 9, 22).and_hms(19, 21, 0),
            Utc.ymd(2021, 12, 21).and_hms(15, 59, 0),
        ],
        2022 => [
            Utc.ymd(2022, 3, 20).and_hms(15, 33, 0),
            Utc.ymd(2022, 6, 21).and_hms(09, 14, 0),
            Utc.ymd(2022, 9, 23).and_hms(01, 04, 0),
            Utc.ymd(2022, 12, 21).and_hms(21, 48, 0),
        ],
        2023 => [
            Utc.ymd(2023, 3, 20).and_hms(21, 25, 0),
            Utc.ymd(2023, 6, 21).and_hms(14, 58, 0),
            Utc.ymd(2023, 9, 23).and_hms(06, 50, 0),
            Utc.ymd(2023, 12, 22).and_hms(03, 28, 0),
        ],
        2024 => [
            Utc.ymd(2024, 3, 20).and_hms(03, 07, 0),
            Utc.ymd(2024, 6, 20).and_hms(20, 51, 0),
            Utc.ymd(2024, 9, 22).and_hms(12, 44, 0),
            Utc.ymd(2024, 12, 21).and_hms(09, 20, 0),
        ],
        2025 => [
            Utc.ymd(2025, 3, 20).and_hms(09, 02, 0),
            Utc.ymd(2025, 6, 21).and_hms(02, 42, 0),
            Utc.ymd(2025, 9, 22).and_hms(18, 20, 0),
            Utc.ymd(2025, 12, 21).and_hms(15, 03, 0),
        ],
        2026 => [
            Utc.ymd(2026, 3, 20).and_hms(14, 46, 0),
            Utc.ymd(2026, 6, 21).and_hms(08, 25, 0),
            Utc.ymd(2026, 9, 23).and_hms(00, 06, 0),
            Utc.ymd(2026, 12, 21).and_hms(20, 50, 0),
        ],
        2027 => [
            Utc.ymd(2027, 3, 20).and_hms(20, 25, 0),
            Utc.ymd(2027, 6, 21).and_hms(14, 11, 0),
            Utc.ymd(2027, 9, 23).and_hms(06, 02, 0),
            Utc.ymd(2027, 12, 22).and_hms(02, 43, 0),
        ],
        2028 => [
            Utc.ymd(2028, 3, 20).and_hms(02, 17, 0),
            Utc.ymd(2028, 6, 20).and_hms(20, 02, 0),
            Utc.ymd(2028, 9, 22).and_hms(11, 45, 0),
            Utc.ymd(2028, 12, 21).and_hms(08, 20, 0),
        ],
        2029 => [
            Utc.ymd(2029, 3, 20).and_hms(08, 01, 0),
            Utc.ymd(2029, 6, 21).and_hms(01, 48, 0),
            Utc.ymd(2029, 9, 22).and_hms(17, 37, 0),
            Utc.ymd(2029, 12, 21).and_hms(14, 14, 0),
        ],
        2030 => [
            Utc.ymd(2030, 3, 20).and_hms(13, 51, 0),
            Utc.ymd(2030, 6, 21).and_hms(07, 31, 0),
            Utc.ymd(2030, 9, 22).and_hms(23, 27, 0),
            Utc.ymd(2030, 12, 21).and_hms(20, 09, 0),
        ],
        2031 => [
            Utc.ymd(2031, 3, 20).and_hms(19, 41, 0),
            Utc.ymd(2031, 6, 21).and_hms(13, 17, 0),
            Utc.ymd(2031, 9, 23).and_hms(05, 15, 0),
            Utc.ymd(2031, 12, 22).and_hms(01, 56, 0),
        ],
        2032 => [
            Utc.ymd(2032, 3, 20).and_hms(01, 23, 0),
            Utc.ymd(2032, 6, 20).and_hms(19, 09, 0),
            Utc.ymd(2032, 9, 22).and_hms(11, 11, 0),
            Utc.ymd(2032, 12, 21).and_hms(07, 57, 0),
        ],
        2033 => [
            Utc.ymd(2033, 3, 20).and_hms(07, 23, 0),
            Utc.ymd(2033, 6, 21).and_hms(01, 01, 0),
            Utc.ymd(2033, 9, 22).and_hms(16, 52, 0),
            Utc.ymd(2033, 12, 21).and_hms(13, 45, 0),
        ],
        2034 => [
            Utc.ymd(2034, 3, 20).and_hms(13, 18, 0),
            Utc.ymd(2034, 6, 21).and_hms(06, 45, 0),
            Utc.ymd(2034, 9, 22).and_hms(22, 41, 0),
            Utc.ymd(2034, 12, 21).and_hms(19, 35, 0),
        ],
        2035 => [
            Utc.ymd(2035, 3, 20).and_hms(19, 03, 0),
            Utc.ymd(2035, 6, 21).and_hms(12, 33, 0),
            Utc.ymd(2035, 9, 23).and_hms(04, 39, 0),
            Utc.ymd(2035, 12, 22).and_hms(01, 31, 0),
        ],
        2036 => [
            Utc.ymd(2036, 3, 20).and_hms(01, 02, 0),
            Utc.ymd(2036, 6, 20).and_hms(18, 31, 0),
            Utc.ymd(2036, 9, 22).and_hms(10, 23, 0),
            Utc.ymd(2036, 12, 21).and_hms(07, 12, 0),
        ],
        2037 => [
            Utc.ymd(2037, 3, 20).and_hms(06, 50, 0),
            Utc.ymd(2037, 6, 21).and_hms(00, 22, 0),
            Utc.ymd(2037, 9, 22).and_hms(16, 13, 0),
            Utc.ymd(2037, 12, 21).and_hms(13, 08, 0),
        ],
        2038 => [
            Utc.ymd(2038, 3, 20).and_hms(12, 40, 0),
            Utc.ymd(2038, 6, 21).and_hms(06, 09, 0),
            Utc.ymd(2038, 9, 22).and_hms(22, 02, 0),
            Utc.ymd(2038, 12, 21).and_hms(19, 01, 0),
        ],
        2039 => [
            Utc.ymd(2039, 3, 20).and_hms(18, 32, 0),
            Utc.ymd(2039, 6, 21).and_hms(11, 58, 0),
            Utc.ymd(2039, 9, 23).and_hms(03, 50, 0),
            Utc.ymd(2039, 12, 22).and_hms(00, 41, 0),
        ],
        2040 => [
            Utc.ymd(2040, 3, 20).and_hms(00, 11, 0),
            Utc.ymd(2040, 6, 20).and_hms(17, 46, 0),
            Utc.ymd(2040, 9, 22).and_hms(09, 44, 0),
            Utc.ymd(2040, 12, 21).and_hms(06, 33, 0),
        ],
        2041 => [
            Utc.ymd(2041, 3, 20).and_hms(06, 07, 0),
            Utc.ymd(2041, 6, 20).and_hms(23, 37, 0),
            Utc.ymd(2041, 9, 22).and_hms(15, 27, 0),
            Utc.ymd(2041, 12, 21).and_hms(12, 19, 0),
        ],
        2042 => [
            Utc.ymd(2042, 3, 20).and_hms(11, 53, 0),
            Utc.ymd(2042, 6, 21).and_hms(05, 16, 0),
            Utc.ymd(2042, 9, 22).and_hms(21, 11, 0),
            Utc.ymd(2042, 12, 21).and_hms(18, 04, 0),
        ],
        2043 => [
            Utc.ymd(2043, 3, 20).and_hms(17, 29, 0),
            Utc.ymd(2043, 6, 21).and_hms(10, 59, 0),
            Utc.ymd(2043, 9, 23).and_hms(03, 07, 0),
            Utc.ymd(2043, 12, 22).and_hms(00, 02, 0),
        ],
        2044 => [
            Utc.ymd(2044, 3, 19).and_hms(23, 20, 0),
            Utc.ymd(2044, 6, 20).and_hms(16, 50, 0),
            Utc.ymd(2044, 9, 22).and_hms(08, 47, 0),
            Utc.ymd(2044, 12, 21).and_hms(05, 43, 0),
        ],
        2045 => [
            Utc.ymd(2045, 3, 20).and_hms(05, 08, 0),
            Utc.ymd(2045, 6, 20).and_hms(22, 34, 0),
            Utc.ymd(2045, 9, 22).and_hms(14, 33, 0),
            Utc.ymd(2045, 12, 21).and_hms(11, 36, 0),
        ],
        2046 => [
            Utc.ymd(2046, 3, 20).and_hms(10, 58, 0),
            Utc.ymd(2046, 6, 21).and_hms(04, 15, 0),
            Utc.ymd(2046, 9, 22).and_hms(20, 22, 0),
            Utc.ymd(2046, 12, 21).and_hms(17, 28, 0),
        ],
        2047 => [
            Utc.ymd(2047, 3, 20).and_hms(16, 52, 0),
            Utc.ymd(2047, 6, 21).and_hms(10, 02, 0),
            Utc.ymd(2047, 9, 23).and_hms(02, 07, 0),
            Utc.ymd(2047, 12, 21).and_hms(23, 07, 0),
        ],
        2048 => [
            Utc.ymd(2048, 3, 19).and_hms(22, 34, 0),
            Utc.ymd(2048, 6, 20).and_hms(15, 54, 0),
            Utc.ymd(2048, 9, 22).and_hms(08, 01, 0),
            Utc.ymd(2048, 12, 21).and_hms(05, 02, 0),
        ],
        2049 => [
            Utc.ymd(2049, 3, 20).and_hms(04, 28, 0),
            Utc.ymd(2049, 6, 20).and_hms(21, 47, 0),
            Utc.ymd(2049, 9, 22).and_hms(13, 42, 0),
            Utc.ymd(2049, 12, 21).and_hms(10, 51, 0),
        ],
        2050 => [
            Utc.ymd(2050, 3, 20).and_hms(10, 20, 0),
            Utc.ymd(2050, 6, 21).and_hms(03, 33, 0),
            Utc.ymd(2050, 9, 22).and_hms(19, 29, 0),
            Utc.ymd(2050, 12, 21).and_hms(16, 39, 0),
        ],
        _ => panic!(),
    }
}
