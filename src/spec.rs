use crate::{eight::Season, YearSeason};
use anyhow::{anyhow, ensure};
use chrono::{Datelike, IsoWeek, Local, Month, NaiveDate, Weekday};
use std::str::FromStr;

enum DateObject {
    Year(i32),
    Month(i32, Month),
    Week(i32, u8),
    Day(NaiveDate),
    Season(YearSeason<Season>),
}

impl FromStr for DateObject {
    type Err = anyhow::Error;
    fn from_str(txt: &str) -> anyhow::Result<DateObject> {
        let mut tokens: Vec<&str> = txt.split('-').collect();
        if tokens.is_empty() {
            return Err(anyhow!("Empty string"));
        }
        let year = match tokens[0].parse() {
            Ok(x) if tokens[0].len() == 4 => {
                tokens.remove(0);
                x
            }
            _ => Local::now().year(),
        };
        Ok(if tokens.is_empty() {
            DateObject::Year(year)
        } else if let Ok(month) = tokens[0].parse::<Month>() {
            DateObject::Month(year, month)
        } else if let Ok(season) = tokens[0].parse::<Season>() {
            DateObject::Season(YearSeason { year, season })
        } else if let Some(week) = tokens[0].strip_prefix('w') {
            DateObject::Week(year, week.parse().unwrap())
        } else {
            let month = tokens[0].parse()?;
            let day = tokens[1].parse()?;
            DateObject::Day(NaiveDate::from_ymd_opt(year, month, day).unwrap())
        })
    }
}

fn new_week(year: i32, week: u32) -> anyhow::Result<IsoWeek> {
    Ok(NaiveDate::from_isoywd_opt(year, week, Weekday::Wed)
        .ok_or(anyhow!("There is no w{} in {}", week, year))?
        .iso_week())
}

pub fn parse_one_week(i: &str) -> anyhow::Result<IsoWeek> {
    match i.chars().next() {
        None => Err(anyhow!("Empty string")),
        Some('w' | 'W') => {
            let w = i[1..].parse()?;
            new_week(Local::now().year(), w)
        }
        _ => {
            let y: i32 = i
                .get(..4)
                .ok_or(anyhow!("Expected a 4-digit year"))?
                .parse()?;
            ensure!(i.get(4..6) == Some("-w"));
            let w: u32 = i[6..].parse()?;
            new_week(y, w)
        }
    }
}

#[derive(Clone, Copy)]
pub enum WeeksSpec {
    Single(IsoWeek),
    Range(IsoWeek, IsoWeek),
    From(IsoWeek),
    To(IsoWeek),
}

impl WeeksSpec {
    pub fn range(&self) -> std::ops::RangeInclusive<IsoWeek> {
        let this_week = || Local::now().iso_week();
        match *self {
            WeeksSpec::Single(x) => x..=x,
            WeeksSpec::Range(from, to) => from..=to,
            WeeksSpec::From(from) => from..=this_week(),
            WeeksSpec::To(to) => this_week()..=to,
        }
    }
}

impl FromStr for WeeksSpec {
    type Err = anyhow::Error;
    fn from_str(i: &str) -> anyhow::Result<WeeksSpec> {
        let xs = i.split("..").collect::<Vec<_>>();
        match xs[..] {
            [x] => parse_one_week(x).map(WeeksSpec::Single),
            [from, ""] => parse_one_week(from).map(WeeksSpec::From),
            ["", to] => parse_one_week(to).map(WeeksSpec::To),
            [from, to] => {
                let from = parse_one_week(from)?;
                let to = parse_one_week(to)?;
                Ok(WeeksSpec::Range(from, to))
            }
            _ => Err(anyhow!("Too many weeks given")),
        }
    }
}
