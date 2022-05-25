use chrono::*;
use scal::*;
use std::collections::BTreeMap;
use structopt::StructOpt;
use yansi::Paint;

#[derive(StructOpt)]
struct Opts {
    /// Show the specified week or range of weeks
    spec: Option<WeeksSpec>,
    /// Show the current week
    #[structopt(long)]
    week: bool,
    /// Show the current year
    #[structopt(long)]
    year: bool,
    /// Show the current month
    #[structopt(long)]
    month: bool,
}

fn parse_event(x: std::io::Result<String>) -> (IsoWeek, String) {
    let x = x.unwrap();
    let (week, event) = x.split_once(' ').unwrap();
    (
        scal::spec::parse_one_week(week).unwrap(),
        event.trim().to_string(),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    use std::io::BufRead;
    let mut events = BTreeMap::<IsoWeek, Vec<String>>::default();
    if let Ok(f) = std::fs::File::open(dirs::config_dir().unwrap().join("scal/events")) {
        for (week, ev) in std::io::BufReader::new(f).lines().map(parse_event) {
            events.entry(week).or_default().push(ev);
        }
    }

    println!("       │ Mo Tu We Th Fr   Sa Su");
    println!("───────┼───────────────────────");
    let range = if let Some(spec) = opts.spec {
        spec.range()
    } else if opts.year {
        let year = Utc::today().year();
        let start = NaiveDate::from_ymd(year, 1, 1);
        let end = NaiveDate::from_ymd(year, 12, 31);
        start.iso_week()..=end.iso_week()
    } else if opts.month {
        let year = Utc::today().year();
        let month = Utc::today().month();
        let start = NaiveDate::from_ymd(year, month, 1);
        let end = NaiveDate::from_ymd(year, month + 1, 1).pred();
        start.iso_week()..=end.iso_week()
    } else if opts.week {
        let this_week = Utc::today().naive_local().iso_week();
        this_week..=this_week
    } else {
        let today = Utc::today().naive_local();
        let start = today - Duration::weeks(2);
        let end = today + Duration::weeks(2);
        start.iso_week()..=end.iso_week()
    };
    for week in weeks_in_range(range) {
        print!("{}", PrettyWeek::new(week));
        if let Some(evs) = events.get(&week) {
            let evs = evs.join(" ▪ ");
            if Utc::today().iso_week() == week {
                print!("  {}", evs);
            } else {
                print!("  {}", Paint::new(evs).dimmed());
            }
        }
        println!();
    }

    Ok(())
}

fn weeks_in_range(range: std::ops::RangeInclusive<IsoWeek>) -> impl Iterator<Item = IsoWeek> {
    let start = NaiveDate::from_isoywd(range.start().year(), range.start().week(), Weekday::Wed);
    let end = *range.end();
    start
        .iter_weeks()
        .map(|x| x.iso_week())
        .take_while(move |x| *x <= end)
}
