pub use crate::{spec::*, *};
use chrono::{Datelike, Local};
use std::fmt::{self, Display};

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
        Season::from_week(Local::now().date_naive().iso_week().week())
    }

    pub fn starting_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 0,
            Spring => 7,
            Summer => 20,
            Autumn => 33,
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

impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?} │ Mo Tu We Th Fr   Sa Su", self)?;
        writeln!(f, "───────┼───────────────────────")?;
        let today = Local::now().date_naive();
        let starting_week = self.starting_week();
        for week in self.weeks() {
            write!(
                f,
                "{}",
                PrettyWeek {
                    year: today.year(),
                    week,
                    starting_week,
                    today,
                }
            )?;
            writeln!(f)?;
        }
        Ok(())
    }
}
