use chrono::*;
use wcal::*;
use std::collections::BTreeMap;
use structopt::StructOpt;
use yansi::Paint;

#[derive(StructOpt)]
struct Opts {
    /// Show the specified week or range of weeks
    spec: Option<WeeksSpec>,
    /// Show the current week
    #[structopt(long, short)]
    week: bool,
    /// Show the current year
    #[structopt(long, short)]
    year: bool,
    /// Show the current month
    #[structopt(long)]
    month: bool,
    /// Show the current season
    #[structopt(long)]
    season: bool,
    /// Don't break on seasons
    #[structopt(long, short)]
    continuous: bool,
    /// Number weeks relative to the season
    #[structopt(long)]
    relative: bool,
    /// Disable colour output
    #[structopt(long)]
    no_color: bool,
}

fn parse_event(x: std::io::Result<String>) -> Option<(IsoWeek, String)> {
    let x = x.unwrap();
    if x.is_empty() || x.starts_with("#") || x.starts_with("//") {
        None
    } else {
        let (week, event) = x.split_once(' ').unwrap();
        Some((
            wcal::spec::parse_one_week(week).unwrap(),
            event.trim().to_string(),
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    if opts.no_color {
        yansi::Paint::disable();
    }

    use std::io::BufRead;
    let mut events = BTreeMap::<IsoWeek, Vec<String>>::default();
    if let Ok(f) = std::fs::File::open(dirs::config_dir().unwrap().join("wcal/events")) {
        for (week, ev) in std::io::BufReader::new(f).lines().flat_map(parse_event) {
            events.entry(week).or_default().push(ev);
        }
    }

    let range = if let Some(spec) = opts.spec {
        spec.range()
    } else if opts.year {
        let year = Utc::today().year();
        let (start, end) = if opts.continuous {
            (
                NaiveDate::from_ymd(year, 1, 1),
                NaiveDate::from_ymd(year, 12, 31),
            )
        } else {
            (
                NaiveDate::from_ymd(year, 1, 4),
                NaiveDate::from_ymd(year, 12, 31),
            )
        };
        start.iso_week()..=end.iso_week()
    } else if opts.season {
        wcal::eight::YearSeason::now().weeks()
    } else if opts.month {
        let year = Utc::today().year();
        let month = Utc::today().month();
        let start = NaiveDate::from_ymd(year, month, 1);
        let end = NaiveDate::from_ymd(year, month + 1, 1).pred();
        start.iso_week()..=end.iso_week()
    } else if opts.week {
        let this_week = Utc::today().naive_local().iso_week();
        this_week..=this_week
    } else if opts.continuous {
        let today = Utc::today().naive_local();
        let start = today - Duration::weeks(3);
        let end = today + Duration::weeks(9);
        start.iso_week()..=end.iso_week()
    } else {
        let this = wcal::eight::YearSeason::now();
        (*this.weeks().start())..=(*this.succ().weeks().end())
    };

    if opts.continuous {
        println!("       │ Mo Tu We Th Fr   Sa Su");
        println!("───────┼───────────────────────");
    }
    let mut season = None;
    for week in weeks_in_range(range) {
        if !opts.continuous {
            let s = wcal::eight::Season::from_week(week.week());
            if season != Some(s) {
                let sname = format!("{s:?}");
                if season.is_some() {
                    println!();
                }
                println!("{sname:>6} │ Mo Tu We Th Fr   Sa Su");
                println!("───────┼───────────────────────");
                season = Some(s);
            }
        }
        let mut pretty_week = PrettyWeek::new(week);
        if opts.relative {
            if let Some(season) = season {
                pretty_week.starting_week = season.starting_week();
            }
        }
        print!("{pretty_week}");
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
