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
    pub fn starting_week(self) -> u8 {
        use Season::*;
        match self {
            Winter => 0,
            Lent => 7,
            Spring => 13,
            Tsuyu => 20,
            Summer => 26,
            Obon => 33,
            Autumn => 39,
            Advent => 46,
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
