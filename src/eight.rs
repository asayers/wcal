use crate::Seasonlike;
use anyhow::anyhow;
use std::str::FromStr;

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
}

impl Seasonlike for Season {
    fn starting_week(self) -> u8 {
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

    fn ending_week(self) -> u8 {
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

    fn from_week(week: u32) -> Season {
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

    fn prev(self) -> Season {
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

    fn succ(self) -> Season {
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
