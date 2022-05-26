use anyhow::anyhow;
use chrono::{Datelike, IsoWeek, NaiveDate, Utc, Weekday};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct YearSeason {
    pub year: i32,
    pub season: Season,
}

impl YearSeason {
    pub fn now() -> YearSeason {
        let today = Utc::today();
        YearSeason {
            year: today.year(),
            season: Season::from_week(today.iso_week().week()),
        }
    }

    pub fn weeks(self) -> std::ops::RangeInclusive<IsoWeek> {
        let start =
            NaiveDate::from_isoywd(self.year, self.season.starting_week() as u32, Weekday::Mon)
                .iso_week();
        let end = NaiveDate::from_isoywd(self.year, self.season.ending_week() as u32, Weekday::Mon)
            .iso_week();
        start..=end
    }

    pub fn from_week(week: IsoWeek) -> YearSeason {
        YearSeason {
            year: week.year(),
            season: Season::from_week(week.week()),
        }
    }

    pub fn prev(self) -> YearSeason {
        YearSeason {
            year: if self.season == Season::Winter {
                self.year - 1
            } else {
                self.year
            },
            season: self.season.prev(),
        }
    }

    pub fn succ(self) -> YearSeason {
        YearSeason {
            year: if self.season == Season::Advent {
                self.year + 1
            } else {
                self.year
            },
            season: self.season.succ(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn starting_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 1,
            Lent => 8,
            Spring => 14,
            Tsuyu => 21,
            Summer => 27,
            Obon => 34,
            Autumn => 40,
            Advent => 47,
        }
    }

    pub fn ending_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 7,
            Lent => 13,
            Spring => 20,
            Tsuyu => 26,
            Summer => 33,
            Obon => 39,
            Autumn => 46,
            Advent => 52,
        }
    }

    pub fn from_week(week: u32) -> Season {
        match week % 52 {
            0 => Season::Advent,
            1..=7 => Season::Winter,
            8..=13 => Season::Lent,
            14..=20 => Season::Spring,
            21..=26 => Season::Tsuyu,
            27..=33 => Season::Summer,
            34..=39 => Season::Obon,
            40..=46 => Season::Autumn,
            47..=51 => Season::Advent,
            _ => unreachable!(),
        }
    }

    pub fn prev(self) -> Season {
        use Season::*;
        match self {
            Lent => Winter,
            Spring => Lent,
            Tsuyu => Spring,
            Summer => Tsuyu,
            Obon => Summer,
            Autumn => Obon,
            Advent => Autumn,
            Winter => Advent,
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

impl FromStr for Season {
    type Err = anyhow::Error;
    fn from_str(txt: &str) -> anyhow::Result<Season> {
        match txt {
            "Winter" => Ok(Season::Winter),
            "Lent" => Ok(Season::Lent),
            "Spring" => Ok(Season::Spring),
            "Tsuyu" => Ok(Season::Tsuyu),
            "Summer" => Ok(Season::Summer),
            "Obon" => Ok(Season::Obon),
            "Autumn" => Ok(Season::Autumn),
            "Advent" => Ok(Season::Advent),
            _ => Err(anyhow!("Unknown: {txt}")),
        }
    }
}

/*
impl Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let season_name = format!("{:?}", self);
        writeln!(f, "   ┌───────┤{:^8}├──────┐", season_name)?;
        writeln!(f, "   │ Mo Tu We Th Fr  Sa Su │")?;
        writeln!(f, "───┼───────────────────────┤")?;
        Ok(())
    }
}

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
*/
