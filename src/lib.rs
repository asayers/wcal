use chrono::{Datelike, Month, NaiveDate, Utc, Weekday};
use enum_map::Enum;
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use yansi::{Color, Paint};

#[derive(Debug, PartialEq, Clone, Copy, Enum)]
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
        1 | 2 | 3 => Color::Blue,
        4 | 5 | 6 => Color::Green,
        7 | 8 | 9 => Color::Yellow,
        10 | 11 | 12 => Color::Red,
        _ => panic!(),
    }
}

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?} │ Mo Tu We Th Fr  Sa Su", self)?;
        writeln!(f, "───────┼──────────────────────")?;
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
                    write!(f, " ")?;
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
