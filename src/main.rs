use bpaf::Bpaf;
use chrono::*;
use std::collections::BTreeMap;
use wcal::*;
use yansi::Paint;

#[derive(Bpaf)]
#[bpaf(options, fallback_to_usage)]
struct Opts {
    /// Show the specified week or range of weeks
    spec: Option<WeeksSpec>,
    /// Show the current week
    #[bpaf(long, short)]
    week: bool,
    /// Show the current year
    #[bpaf(long, short)]
    year: bool,
    /// Show the current month
    month: bool,
    /// Show the current season
    season: bool,
    /// Don't break on seasons
    #[bpaf(long, short)]
    continuous: bool,
    /// Break on months
    months: bool,
    /// Number weeks relative to the season
    relative: bool,
    /// Disable colour output
    no_color: bool,
    /// Print the date and exit
    date: bool,
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

#[derive(PartialEq, Eq)]
enum Grouping {
    None,
    Months,
    Seasons,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = opts().run();

    if opts.date {
        let season = wcal::YearSeason::<wcal::eight::Season>::now().season;
        let week =
            Local::now().date_naive().iso_week().week() - u32::from(season.starting_week()) + 1;
        let day = Local::now().date_naive().weekday();
        println!("{season:?}-{week} ({day})");
        return Ok(());
    }

    if opts.no_color {
        yansi::Paint::disable();
    }

    let grouping = match (opts.continuous, opts.months) {
        (true, true) => return Err("Can't pass both --months and --continuous".into()),
        (true, false) => Grouping::None,
        (false, true) => Grouping::Months,
        (false, false) => Grouping::Seasons,
    };

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
        let year = Local::now().date_naive().year();
        let (start, end) = if grouping == Grouping::None {
            (
                NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(year, 12, 31).unwrap(),
            )
        } else {
            (
                NaiveDate::from_ymd_opt(year, 1, 4).unwrap(),
                NaiveDate::from_ymd_opt(year, 12, 31).unwrap(),
            )
        };
        start.iso_week()..=end.iso_week()
    } else if opts.season {
        wcal::YearSeason::<wcal::eight::Season>::now().weeks()
    } else if opts.month {
        let year = Local::now().date_naive().year();
        let month = Local::now().date_naive().month();
        let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap();
        start.iso_week()..=end.iso_week()
    } else if opts.week {
        let this_week = Local::now().date_naive().iso_week();
        this_week..=this_week
    } else if grouping == Grouping::None {
        let today = Local::now().date_naive();
        let start = today - Duration::weeks(3);
        let end = today + Duration::weeks(9);
        start.iso_week()..=end.iso_week()
    } else {
        let this = wcal::YearSeason::<wcal::eight::Season>::now();
        (*this.weeks().start())..=(*this.succ().weeks().end())
    };

    if grouping == Grouping::None {
        println!("       │ Mo Tu We Th Fr   Sa Su");
        println!("───────┼───────────────────────");
    }
    let mut season = None;
    let mut month = None;
    for week in weeks_in_range(range) {
        match grouping {
            Grouping::Seasons => {
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
            Grouping::None => (),
            Grouping::Months => {
                let m = week_to_month(week);
                if month != Some(m) {
                    let mname = &format!("{m:?}")[..3];
                    if month.is_some() {
                        println!();
                    }
                    println!("{mname:>6} │ Mo Tu We Th Fr   Sa Su");
                    println!("───────┼───────────────────────");
                    month = Some(m);
                }
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
            if Local::now().date_naive().iso_week() == week {
                print!("  {}", evs);
            } else {
                print!("  {}", Paint::new(evs).dimmed());
            }
        }
        println!();
    }

    Ok(())
}

fn week_to_month(week: IsoWeek) -> Month {
    let m = NaiveDate::from_isoywd_opt(week.year(), week.week(), Weekday::Sun)
        .unwrap()
        .month();
    Month::try_from(u8::try_from(m).unwrap()).unwrap()
}

fn weeks_in_range(range: std::ops::RangeInclusive<IsoWeek>) -> impl Iterator<Item = IsoWeek> {
    let start =
        NaiveDate::from_isoywd_opt(range.start().year(), range.start().week(), Weekday::Wed)
            .unwrap();
    let end = *range.end();
    start
        .iter_weeks()
        .map(|x| x.iso_week())
        .take_while(move |x| *x <= end)
}
