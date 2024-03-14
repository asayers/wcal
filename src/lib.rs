pub mod eight;
pub mod four;
pub mod spec;

pub use crate::spec::*;
use chrono::{Datelike, IsoWeek, Local, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use yansi::{Color, Paint};

pub trait Seasonlike: PartialEq + Copy {
    fn starting_week(self) -> u8;
    fn ending_week(self) -> u8;
    fn from_week(week: u32) -> Self;
    fn prev(self) -> Self;
    fn succ(self) -> Self;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct YearSeason<T> {
    pub year: i32,
    pub season: T,
}

impl<T: Seasonlike> YearSeason<T> {
    pub fn now() -> YearSeason<T> {
        let today = Local::now().date_naive();
        YearSeason {
            year: today.year(),
            season: T::from_week(today.iso_week().week()),
        }
    }

    pub fn weeks(self) -> std::ops::RangeInclusive<IsoWeek> {
        let start =
            NaiveDate::from_isoywd_opt(self.year, self.season.starting_week() as u32, Weekday::Mon)
                .unwrap()
                .iso_week();
        let end =
            NaiveDate::from_isoywd_opt(self.year, self.season.ending_week() as u32, Weekday::Mon)
                .unwrap()
                .iso_week();
        start..=end
    }

    pub fn from_week(week: IsoWeek) -> YearSeason<T> {
        YearSeason {
            year: week.year(),
            season: T::from_week(week.week()),
        }
    }

    pub fn prev(self) -> YearSeason<T> {
        YearSeason {
            year: if self.season == T::from_week(1) {
                self.year - 1
            } else {
                self.year
            },
            season: self.season.prev(),
        }
    }

    pub fn succ(self) -> YearSeason<T> {
        YearSeason {
            year: if self.season == T::from_week(52) {
                self.year + 1
            } else {
                self.year
            },
            season: self.season.succ(),
        }
    }
}

fn month_colour(month: u32) -> Color {
    match month {
        12 | 1 | 2 => Color::Blue,
        3 | 4 | 5 => Color::Green,
        6 | 7 | 8 => Color::Yellow,
        9 | 10 | 11 => Color::Red,
        _ => panic!(),
    }
}

pub struct PrettyWeek {
    pub year: i32,
    pub week: u8,
    pub starting_week: u8,
    pub today: NaiveDate,
}

impl PrettyWeek {
    pub fn new(week: IsoWeek) -> PrettyWeek {
        PrettyWeek {
            year: week.year(),
            week: week.week() as u8,
            starting_week: 1,
            today: Local::now().date_naive(),
        }
    }
}

impl Display for PrettyWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut new_month = None;
        let this_week = self.today.iso_week();
        let weeknum = self.week - self.starting_week + 1;
        if self.week == this_week.week() as u8 && self.year == this_week.year() {
            let x = format!(" ▶ w{weeknum:02}");
            write!(f, "{}", Paint::new(x).bold())?;
        } else {
            write!(f, "   w{weeknum:02}")?;
        }
        write!(f, " │")?;
        for &day in &[
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ] {
            let date = if self.week == 0 {
                NaiveDate::from_isoywd_opt(self.year - 1, 53, day).unwrap()
            } else {
                NaiveDate::from_isoywd_opt(self.year, self.week as u32, day).unwrap()
            };
            let color = month_colour(date.month());
            let dimmed = date.month() % 2 == 0;
            if date.day() == 1 {
                new_month = Some(date.month());
            }
            if day == Weekday::Sat {
                write!(f, "  ")?;
            }
            if date == self.today {
                write!(f, " {}", Paint::new(format!("{:2}", date.day())).bold())?;
            } else if dimmed {
                write!(f, " {}", color.paint(format!("{:2}", date.day())).dimmed())?;
            } else {
                write!(f, " {}", color.paint(format!("{:2}", date.day())))?;
            }
        }
        if let Some(m) = new_month {
            let color = month_colour(m);
            let dimmed = m % 2 == 0;
            let x = &Month::from_u32(m).unwrap().name()[..3];
            if dimmed {
                write!(f, "  {} ", color.paint(x).dimmed())?;
            } else {
                write!(f, "  {} ", color.paint(x))?;
            }
        } else {
            write!(f, "      ")?;
        }
        Ok(())
    }
}
