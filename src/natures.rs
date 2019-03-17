use crate::enums::*;
use crate::moves::BattleStyle;
use crate::items::Flavor;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use crate::FromVeekun;

/// A Pokémon's nature affects its stats and some berry effects.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Nature)
/// > Natures (Japanese: 性格 nature) are a mechanic that influences how a
/// > Pokémon's stats grow. They were introduced in Generation III. . . .
/// >
/// > A Pokémon's Nature usually affects the value of two of its stats,
/// > ultimately increasing one of its non-HP stats (Attack, Defense, Special
/// > Attack, Special Defense, or Speed) by 10% and decreasing another by 10%.
/// > Starting in Pokémon HeartGold and SoulSilver, the stats increased or
/// > decreased by a Pokémon's Nature have (respectively) a red or blue
/// > highlight on a Pokémon's summary screen.
/// > 
/// > Natures also determine the Pokémon's favorite flavor and its disliked
/// > flavor: Each stat is associated with a flavor, and each Pokémon's favorite
/// > flavor is the one associated with the stat increased by its Nature, while
/// > it dislikes the flavor associated with the decreased stat. For example,
/// > since the Lonely Nature increases a Pokémon's Attack stat (which is
/// > associated with the Spicy flavor) and decreases its Defense stat (which is
/// > associated with the Sour flavor), a Pokémon with a Lonely Nature has Spicy
/// > as its favorite flavor and dislikes Sour flavors.
/// > 
/// > Every Nature represents one of the 25 unique possible combinations of stat
/// > increase and decrease; thus, there are five Natures that have no effect on
/// > the Pokémon's stat growth as they technically increase and decrease the
/// > same stat (Bashful, Docile, Hardy, Quirky, and Serious).
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

impl Nature {
    /// Get which flavor is disliked, if any.
    pub fn disliked(self) -> Option<Flavor> {
        let x = self.repr();
        if x % 6 == 0 {
            return None;
        }
        Flavor::from_repr(x / 5).or_else(|| unreachable!())
    }

    /// Get which stat is increased, if any.
    pub fn increased(self) -> Option<Stat> {
        self.disliked().and_then(|x|
            Stat::from_repr(x.repr() as i8).or_else(|| unreachable!()))
    }

    /// Get which stat is decreased, if any.
    pub fn decreased(self) -> Option<Stat> {
        let x = self.repr();
        if x % 6 == 0 {
            return None;
        }
        Stat::from_repr((x % 5) as i8).or_else(|| unreachable!())
    }
}

impl Default for Nature {
    fn default() -> Self { Nature::Hardy }
}

impl FromVeekun for Nature {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
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

/// Stats are numbers associated with Pokémon affecting battle mechanics.
///
/// Some stats are only available in battle, whereas others are properties of
/// the Pokémon.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Statistic)
/// > A statistic (Japanese: 能力 ability), or stat for short, is an element
/// > which determines certain aspects of battles in the games. Stats may also
/// > refer to the numerical values of each field in regards to individual
/// > Pokémon.
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

impl FromVeekun for Stat {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
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

/// Half of the table determining Battle Palace behavior. See `PalaceTable`.
#[derive(Default)]
pub struct HalfPalaceTable {
    pub attack: [u8; Nature::COUNT],
    pub defense: [u8; Nature::COUNT],
}

impl HalfPalaceTable {
    /// Randomly select a battle style based on the weights in the table.
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

/// Table of probabilities determining Battle Palace behavior.
///
/// There are two half tables, one for when HP is below half, one for otherwise.
#[derive(Default)]
pub struct PalaceTable {
    pub low: HalfPalaceTable,
    pub high: HalfPalaceTable,
}

impl PalaceTable {
    /// Create a palace table from the included Veekun CSV data.
    pub fn new() -> Self {
        Self::from_csv_data(vdata::PALACE).unwrap()
    }
}

impl vcsv::FromCsvIncremental for PalaceTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
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
                let error = vcsv::MiscError::from("Preferences must sum to 100.");
                if low_attack + low_defense + low != 100 {
                    return Err(vcsv::Error::Veekun {
                        line: Some(line),
                        field: 2,
                        error: Box::new(error),
                    });
                }
                if high_attack + high_defense + high != 100 {
                    return Err(vcsv::Error::Veekun {
                        line: Some(line),
                        field: 3,
                        error: Box::new(error),
                    });
                }
            }
        }
        Ok(())
    }
}
