use crate::enums::*;
use crate::FromVeekun;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;

/// Level of efficacy of some type combination.
#[EnumRepr(type = "i8")]
pub enum Efficacy {
    /// Attacks have no effect.
    Not = -2,
    /// Attacks do half damage.
    NotVery,
    /// Attacks do regular damage.
    Regular,
    /// Attacks do double damage.
    Super,
}

impl Default for Efficacy {
    fn default() -> Self { Efficacy::Regular }
}

impl FromVeekun for Efficacy {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
            0 => Some(Efficacy::Not),
            50 => Some(Efficacy::NotVery),
            100 => Some(Efficacy::Regular),
            200 => Some(Efficacy::Super),
            _ => None,
        }
    }
}

/// The type of a Pokémon or move.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Type) Types
/// > (Japanese: タイプ Type) are properties for Pokémon and their moves. . . .
/// >
/// > A Pokémon may have either one or two types: For instance, Charmander is a
/// > Fire type, while Bulbasaur is both a Grass type and a Poison type. . . .
/// >
/// > A move has exactly one type. The type of a damaging move typically defines
/// > which types of Pokémon it is super effective against, which types of
/// > Pokémon it is not very effective against, and which types of Pokémon it is
/// > completely ineffective against.
#[EnumRepr(type = "u8")]
pub enum Type {
    Normal = 0,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}

impl Default for Type {
    fn default() -> Self { Type::Normal }
}

impl FromVeekun for Type {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

/// Table of the efficacies of type combinations.
#[derive(Default)]
pub struct EfficacyTable([[Efficacy; Type::COUNT]; Type::COUNT]);

impl EfficacyTable {
    /// Creates a type efficacy table from the included Veekun CSV data.
    pub fn new() -> Self {
        Self::from_csv_data(vdata::EFFICACY).unwrap()
    }
}

impl vcsv::FromCsvIncremental for EfficacyTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let damage = vcsv::from_field(&record, 0)?;
        let target = vcsv::from_field(&record, 1)?;
        let efficacy = vcsv::from_field(&record, 2)?;
        self[(damage, target)] = efficacy;
        Ok(())
    }
}

impl std::ops::Index<(Type, Type)> for EfficacyTable {
    type Output = Efficacy;

    /// Get the efficacy of a (damage, target) type combination.
    fn index<'a>(&'a self, index: (Type, Type)) -> &'a Efficacy {
        &self.0[index.0.repr() as usize][index.1.repr() as usize]
    }
}

impl std::ops::IndexMut<(Type, Type)> for EfficacyTable {
    /// Access the efficacy of a (damage, target) type combination mutably.
    fn index_mut<'a>(&'a mut self, index: (Type, Type)) -> &'a mut Efficacy {
        &mut self.0[index.0.repr() as usize][index.1.repr() as usize]
    }
}
