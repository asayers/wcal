use chrono::{Datelike, Month, NaiveDate, Utc, Weekday};
use num_traits::FromPrimitive;
use season_date::*;
use yansi::{Color, Paint};

fn main() {
    print_all()
}

fn print_header(season: Season) {
    println!("   ┌───────┤{:^8}├──────┐", format!("{:?}", season));
    println!("   │ Mo Tu We Th Fr  {} │", Paint::new("Sa Su").dimmed());
    println!("───┼───────────────────────┤");
}
fn print_footer() {
    println!("───┴───────────────────────┘");
}

fn print_all() {
    let today = Utc::today();
    let year = today.year();

    let mut season = Season::Winter;
    print_header(season);
    for week in 1..=52 {
        if season.succ().starting_week() == week {
            season = season.succ();
            print_footer();
            println!();
            print_header(season);
        }
        let mut new_month = None;
        print!("w{} │", week - season.starting_week() + 1);
        for &day in &[
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ] {
            let date = NaiveDate::from_isoywd(year, week.into(), day);
            let color = if date.month() % 2 == 0 {
                Color::Blue
            } else {
                Color::Magenta
            };
            if date.day() == 1 {
                new_month = Some(date.month());
            }
            if day == Weekday::Sat {
                print!(" ");
            }
            if date == today.naive_local() {
                print!(" {}", Paint::new(format!("{:2}", date.day())).bold());
            } else if day == Weekday::Sat || day == Weekday::Sun {
                print!(" {}", color.paint(format!("{:2}", date.day())).dimmed());
            } else {
                print!(" {}", color.paint(format!("{:2}", date.day())));
            }
        }
        print!(" │");
        if let Some(m) = new_month {
            let color = if m % 2 == 0 {
                Color::Blue
            } else {
                Color::Magenta
            };
            print!("  {:?}", color.paint(Month::from_u32(m).unwrap()));
        }
        println!();
    }
    print_footer();
}

// fn print_week(year: i32, season: Season, week: u8, today: Date<Utc>) {
//     print!("w{:02} |", week);
//     let mut new_month = None;
//     for &day in &[
//         Weekday::Mon,
//         Weekday::Tue,
//         Weekday::Wed,
//         Weekday::Thu,
//         Weekday::Fri,
//         Weekday::Sat,
//         Weekday::Sun,
//     ] {
//         let date = Seasonal {
//             year,
//             season,
//             week,
//             day,
//         }
//         .to_date();
//         let color = if date.month() % 2 == 0 {
//             Color::Blue
//         } else {
//             Color::Magenta
//         };
//         if date.day() == 1 {
//             new_month = Some(date.month());
//         }
//         if date == today.naive_local() {
//             print!(" {}", Paint::new(format!("{:02}", date.day())).bold());
//         } else if day == today.weekday() {
//             print!(" {}", color.paint(format!("{:02}", date.day())).bold());
//         } else {
//             print!(" {}", color.paint(format!("{:02}", date.day())));
//         }
//     }
//     if let Some(m) = new_month {
//         let color = if m % 2 == 0 {
//             Color::Blue
//         } else {
//             Color::Magenta
//         };
//         print!("  {:?}", color.paint(Month::from_u32(m).unwrap()));
//     }
//     println!();
// }
