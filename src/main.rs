use scal::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(long, short)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    if opts.all {
        let cols = 1; // crossterm::terminal::size()?.0 / 40;
        for ss in Season::ALL.chunks(cols as usize) {
            println!();
            let mut lines = 0;
            let ss = ss
                .iter()
                .map(|s| {
                    let x = format!("{}", s)
                        .lines()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    lines = lines.max(x.len());
                    x
                })
                .collect::<Vec<_>>();
            for l in 0..lines {
                for s in &ss {
                    print!("{:<40}", s.get(l).map_or("", |x| x.as_str()));
                }
                println!();
            }
        }
    } else {
        let season = Season::now();
        print!("{}", season);
    }
    Ok(())
}
