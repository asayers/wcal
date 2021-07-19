use anyhow::{anyhow, ensure};
use chrono::{Datelike, IsoWeek, NaiveDate, Utc, Weekday};
use std::str::FromStr;

fn new_week(year: i32, week: u32) -> anyhow::Result<IsoWeek> {
    Ok(NaiveDate::from_isoywd_opt(year, week, Weekday::Wed)
        .ok_or(anyhow!("There is no w{} in {}", week, year))?
        .iso_week())
}

fn parse_one_week(i: &str) -> anyhow::Result<IsoWeek> {
    match i.chars().next() {
        None => Err(anyhow!("Empty string")),
        Some('w' | 'W') => {
            let w = i[1..].parse()?;
            new_week(Utc::today().year(), w)
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
        let this_week = || Utc::today().iso_week();
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
