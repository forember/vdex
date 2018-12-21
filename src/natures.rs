//! Natures, stats, and related data.

use enums::*;
use moves::BattleStyle;
use vcsv;
use FromVeekun;

pub const NATURE_COUNT: usize = 25;

#[EnumRepr(type = "u8")]
pub enum ContestType {
    Cool = 0,
    Tough,
    Cute,
    Beauty,
    Smart,
}

#[EnumRepr(type = "u8")]
pub enum Flavor {
    Spicy = 0,
    Sour,
    Sweet,
    Dry,
    Bitter,
}

#[EnumRepr(type = "u8")]
pub enum Nature {
    Hardy = 0,
    Lonely,
    Brave,
    Adamant,
    Naughty,
    Bold,
    Docile,
    Relaxed,
    Impish,
    Lax,
    Timid,
    Hasty,
    Serious,
    Jolly,
    Naive,
    Modest,
    Mild,
    Quiet,
    Bashful,
    Rash,
    Calm,
    Gentle,
    Sassy,
    Careful,
    Quirky,
}

#[EnumRepr(type = "i8")]
pub enum Stat {
    HP = -1,
    Attack,
    Defense,
    Speed,
    SpecialAttack,
    SpecialDefense,
    Accuracy,
    Evasion,
}

pub fn assert_sanity() {
    assert_eq!(ContestType::Smart.repr(), 4);
    assert_eq!(Flavor::Bitter.repr(), 4);
    assert_eq!(Nature::Quirky.repr(), 24);
    assert_eq!(Stat::Evasion.repr(), 6);
}

impl std::convert::From<Flavor> for ContestType {
    fn from(flavor: Flavor) -> Self {
        ContestType::from_repr(flavor.repr()).unwrap()
    }
}

impl FromVeekun for ContestType {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        match id {
            1 => Some(ContestType::Cool),
            2 => Some(ContestType::Beauty),
            3 => Some(ContestType::Cute),
            4 => Some(ContestType::Smart),
            5 => Some(ContestType::Tough),
            _ => None,
        }
    }
}

impl std::convert::From<ContestType> for Flavor {
    fn from(contest: ContestType) -> Self {
        Flavor::from_repr(contest.repr()).unwrap()
    }
}

impl FromVeekun for Flavor {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        ContestType::from_veekun(id).and_then(|t| Some(Flavor::from(t)))
    }
}

impl Nature {
    pub fn disliked(self) -> Option<Flavor> {
        let x = self.repr();
        if x % 6 == 0 {
            return None;
        }
        Flavor::from_repr(x / 5).or_else(|| unreachable!())
    }

    pub fn increased(self) -> Option<Stat> {
        self.disliked().and_then(|x|
            Stat::from_repr(x.repr() as i8).or_else(|| unreachable!()))
    }

    pub fn decreased(self) -> Option<Stat> {
        let x = self.repr();
        if x % 6 == 0 {
            return None;
        }
        Stat::from_repr((x % 5) as i8).or_else(|| unreachable!())
    }
}

impl FromVeekun for Nature {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        match id {
            1 => Some(Nature::Hardy),
            2 => Some(Nature::Bold),
            3 => Some(Nature::Modest),
            4 => Some(Nature::Calm),
            5 => Some(Nature::Timid),
            6 => Some(Nature::Lonely),
            7 => Some(Nature::Docile),
            8 => Some(Nature::Mild),
            9 => Some(Nature::Gentle),
            10 => Some(Nature::Hasty),
            11 => Some(Nature::Adamant),
            12 => Some(Nature::Impish),
            13 => Some(Nature::Bashful),
            14 => Some(Nature::Careful),
            15 => Some(Nature::Rash),
            16 => Some(Nature::Jolly),
            17 => Some(Nature::Naughty),
            18 => Some(Nature::Lax),
            19 => Some(Nature::Quirky),
            20 => Some(Nature::Naive),
            21 => Some(Nature::Brave),
            22 => Some(Nature::Relaxed),
            23 => Some(Nature::Quiet),
            24 => Some(Nature::Sassy),
            25 => Some(Nature::Serious),
            _ => None,
        }
    }
}

impl FromVeekun for Stat {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        match id {
            1 => Some(Stat::HP),
            2 => Some(Stat::Attack),
            3 => Some(Stat::Defense),
            4 => Some(Stat::SpecialAttack),
            5 => Some(Stat::SpecialDefense),
            6 => Some(Stat::Speed),
            7 => Some(Stat::Accuracy),
            8 => Some(Stat::Evasion),
            _ => None,
        }
    }
}

pub struct HalfPalaceTable {
    pub attack: [u8; NATURE_COUNT],
    pub defense: [u8; NATURE_COUNT],
}

impl HalfPalaceTable {
    pub fn pick_style<R: rand::Rng>(
        &self, rng: &mut R, nature: Nature
    ) -> BattleStyle {
        let i = nature.repr() as usize;
        let a = self.attack[i];
        match rng.gen_range(0, 100) {
            x if x < a => BattleStyle::Attack,
            x if x < a + self.defense[i] => BattleStyle::Defense,
            _ => BattleStyle::Support,
        }
    }
}

pub struct PalaceTable {
    pub low: HalfPalaceTable,
    pub high: HalfPalaceTable,
}

impl vcsv::FromCsvIncremental for PalaceTable {
    fn from_empty_csv() -> Self {
        PalaceTable {
            low: HalfPalaceTable {
                attack: [0; NATURE_COUNT],
                defense: [0; NATURE_COUNT],
            },
            high: HalfPalaceTable {
                attack: [0; NATURE_COUNT],
                defense: [0; NATURE_COUNT],
            },
        }
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let nature: Nature = vcsv::from_field(&record, 0)?;
        let nature_id = nature.repr() as usize;
        let style = vcsv::from_field(&record, 1)?;
        let low = vcsv::from_field(&record, 2)?;
        let high = vcsv::from_field(&record, 3)?;
        match style {
            BattleStyle::Attack => {
                self.low.attack[nature_id] = low;
                self.high.attack[nature_id] = high;
            },
            BattleStyle::Defense => {
                self.low.defense[nature_id] = low;
                self.high.defense[nature_id] = high;
            },
            BattleStyle::Support => {
                let low_attack = self.low.attack[nature_id];
                let high_attack = self.high.attack[nature_id];
                let low_defense = self.low.defense[nature_id];
                let high_defense = self.high.defense[nature_id];
                let line = match record.position() {
                    Some(p) => p.line(),
                    None => 0,
                };
                let debug = "Preferences must sum to 100.";
                if low_attack + low_defense + low != 100 {
                    return Err(vcsv::Error::Veekun {
                        line: Some(line),
                        field: 2,
                        debug: Box::new(debug),
                    });
                }
                if high_attack + high_defense + high != 100 {
                    return Err(vcsv::Error::Veekun {
                        line: Some(line),
                        field: 3,
                        debug: Box::new(debug),
                    });
                }
            }
        }
        Ok(())
    }
}
