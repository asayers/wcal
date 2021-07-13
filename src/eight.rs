use chrono::{Datelike, Month, NaiveDate, Utc, Weekday};
use enum_map::Enum;
use num_traits::FromPrimitive;
use std::fmt::{self, Display};
use yansi::{Color, Paint};

#[derive(Debug, PartialEq, Clone, Copy, Enum)]
pub enum Season {
    Winter,
    Lent,
    Spring,
    Tsuyu,
    Summer,
    Obon,
    Autumn,
    Advent,
}

impl Season {
    pub const ALL: [Season; 8] = [
        Season::Winter,
        Season::Lent,
        Season::Spring,
        Season::Tsuyu,
        Season::Summer,
        Season::Obon,
        Season::Autumn,
        Season::Advent,
    ];

    pub fn now() -> Season {
        Season::from_week(Utc::today().iso_week().week())
    }

    pub fn starting_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 0,
            Lent => 7,
            Spring => 13,
            Tsuyu => 20,
            Summer => 26,
            Obon => 33,
            Autumn => 39,
            Advent => 46,
        }
    }

    pub fn from_week(week: u32) -> Season {
        match week % 52 {
            x if x < 7 => Season::Winter,
            x if x < 13 => Season::Lent,
            x if x < 20 => Season::Spring,
            x if x < 26 => Season::Tsuyu,
            x if x < 33 => Season::Summer,
            x if x < 39 => Season::Obon,
            x if x < 46 => Season::Autumn,
            x if x < 52 => Season::Advent,
            _ => unreachable!(),
        }
    }

    pub fn weeks(self) -> std::ops::Range<u8> {
        if self == Season::Advent {
            self.starting_week()..52
        } else {
            self.starting_week()..self.succ().starting_week()
        }
    }

    pub fn succ(self) -> Season {
        use Season::*;
        match self {
            Winter => Lent,
            Lent => Spring,
            Spring => Tsuyu,
            Tsuyu => Summer,
            Summer => Obon,
            Obon => Autumn,
            Autumn => Advent,
            Advent => Winter,
        }
    }
}

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let season_name = format!("{:?}", self);
        writeln!(f, "   ┌───────┤{:^8}├──────┐", season_name)?;
        writeln!(f, "   │ Mo Tu We Th Fr  Sa Su │")?;
        writeln!(f, "───┼───────────────────────┤")?;
        let today = Utc::today();
        let year = today.year();
        for week in self.weeks() {
            let mut new_month = None;
            write!(f, "w{} │", week - self.starting_week() + 1)?;
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
                let color = if date.month() % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Magenta
                };
                if date.day() == 1 {
                    new_month = Some(date.month());
                }
                if day == Weekday::Sat {
                    write!(f, " ")?;
                }
                if date == today.naive_local() {
                    write!(f, " {}", Paint::new(format!("{:2}", date.day())).bold())?;
                } else {
                    write!(f, " {}", color.paint(format!("{:2}", date.day())))?;
                }
            }
            write!(f, " │")?;
            if let Some(m) = new_month {
                let color = if m % 2 == 0 {
                    Color::Blue
                } else {
                    Color::Magenta
                };
                write!(f, "  {:?}", color.paint(Month::from_u32(m).unwrap()))?;
            }
            writeln!(f)?;
        }
        f.write_str("───┴───────────────────────┘\n")?;
        Ok(())
    }
}
