mod spec;

pub use crate::spec::*;
use chrono::{Datelike, IsoWeek, Month, NaiveDate, Utc, Weekday};
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use yansi::{Color, Paint};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

impl Season {
    pub const ALL: [Season; 4] = [
        Season::Winter,
        Season::Spring,
        Season::Summer,
        Season::Autumn,
    ];

    pub fn now() -> Season {
        Season::from_week(Utc::today().iso_week().week())
    }

    pub fn starting_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 0,
            Spring => 13,
            Summer => 26,
            Autumn => 39,
        }
    }

    pub fn from_week(week: u32) -> Season {
        match week % 52 {
            x if x < 7 => Season::Winter,
            x if x < 20 => Season::Spring,
            x if x < 33 => Season::Summer,
            x if x < 46 => Season::Autumn,
            _ => unreachable!(),
        }
    }

    pub fn weeks(self) -> std::ops::Range<u8> {
        if self == Season::Autumn {
            self.starting_week()..52
        } else {
            self.starting_week()..self.succ().starting_week()
        }
    }

    pub fn succ(self) -> Season {
        use Season::*;
        match self {
            Winter => Spring,
            Spring => Summer,
            Summer => Autumn,
            Autumn => Winter,
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

pub fn print_week(week: IsoWeek, starting_week: u32) {
    let today = Utc::today();
    let mut new_month = None;
    let weeknum = format!("w{:02}", week.week() - starting_week + 1);
    if week == today.iso_week() {
        print!(" > {} │", Paint::new(weeknum).bold());
    } else {
        print!("   {} │", weeknum);
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
        // if week == 0 { NaiveDate::from_isoywd(year - 1, 53, day)
        let date = NaiveDate::from_isoywd(week.year(), week.week(), day);
        let color = month_colour(date.month());
        let dimmed = date.month() % 2 == 0;
        if date.day() == 1 {
            new_month = Some(date.month());
        }
        if day == Weekday::Sat {
            print!("  ");
        }
        if date == today.naive_local() {
            print!(" {}", Paint::new(format!("{:2}", date.day())).bold());
        } else if dimmed {
            print!(" {}", color.paint(format!("{:2}", date.day())).dimmed());
        } else {
            print!(" {}", color.paint(format!("{:2}", date.day())));
        }
    }
    if let Some(m) = new_month {
        let color = month_colour(m);
        let dimmed = m % 2 == 0;
        if dimmed {
            print!("  {:?}", color.paint(Month::from_u32(m).unwrap()).dimmed());
        } else {
            print!("  {:?}", color.paint(Month::from_u32(m).unwrap()));
        }
    }
    println!();
}

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?} │ Mo Tu We Th Fr   Sa Su", self)?;
        writeln!(f, "───────┼───────────────────────")?;
        let today = Utc::today();
        let this_week = today.iso_week().week() as u8;
        let year = today.year();
        for week in self.weeks() {
            let mut new_month = None;
            let x = format!("w{:02}", week - self.starting_week() + 1);
            if week == this_week {
                write!(f, " > {} │", Paint::new(x).bold())?;
            } else {
                write!(f, "   {} │", x)?;
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
                let date = if week == 0 {
                    NaiveDate::from_isoywd(year - 1, 53, day)
                } else {
                    NaiveDate::from_isoywd(year, week.into(), day)
                };
                let color = month_colour(date.month());
                let dimmed = date.month() % 2 == 0;
                if date.day() == 1 {
                    new_month = Some(date.month());
                }
                if day == Weekday::Sat {
                    write!(f, "  ")?;
                }
                if date == today.naive_local() {
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
                if dimmed {
                    write!(
                        f,
                        "  {:?}",
                        color.paint(Month::from_u32(m).unwrap()).dimmed()
                    )?;
                } else {
                    write!(f, "  {:?}", color.paint(Month::from_u32(m).unwrap()))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
