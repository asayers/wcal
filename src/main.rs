use enum_map::Enum;
use season_date::*;

fn main() {
    print!("{}", Season::Winter);
    for i in 1..8 {
        println!();
        print!("{}", <Season as Enum::<()>>::from_usize(i));
    }
}
