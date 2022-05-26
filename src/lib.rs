pub mod eight;
pub mod four;
pub mod spec;

pub use crate::spec::*;
use chrono::{Datelike, IsoWeek, Month, NaiveDate, Utc, Weekday};
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use yansi::{Color, Paint};

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
            today: Utc::today().naive_local(),
        }
    }
}

impl Display for PrettyWeek {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut new_month = None;
        let weeknum = format!("w{:02}", self.week - self.starting_week + 1);
        if self.week == self.today.iso_week().week() as u8
            && self.year == self.today.iso_week().year()
        {
            write!(f, " > {} │", Paint::new(weeknum).bold())?;
        } else {
            write!(f, "   {} │", weeknum)?;
        }
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
                NaiveDate::from_isoywd(self.year - 1, 53, day)
            } else {
                NaiveDate::from_isoywd(self.year, self.week as u32, day)
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
            let x = format!("{:9}", format!("{:?}", Month::from_u32(m).unwrap()));
            if dimmed {
                write!(f, "  {}", color.paint(x).dimmed())?;
            } else {
                write!(f, "  {}", color.paint(x))?;
            }
        } else {
            write!(f, "  {:9}", " ")?;
        }
        Ok(())
    }
}
