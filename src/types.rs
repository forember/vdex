//! Types and type efficacy.

use enums::*;
use vcsv;
use FromVeekun;

pub const TYPE_COUNT: usize = 17;

#[EnumRepr(type = "i8")]
pub enum Efficacy {
    Not = -2,
    NotVery,
    Regular,
    Super,
}

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

pub fn assert_sanity() {
    assert_eq!(Efficacy::Super.repr(), 1);
    assert_eq!(Type::Dark.repr(), 16);
}

impl FromVeekun<u8> for Efficacy {
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

impl FromVeekun<u8> for Type {
    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Type::from_repr)
    }
}

pub struct EfficacyTable {
    pub table: [Efficacy; TYPE_COUNT*TYPE_COUNT],
}

impl EfficacyTable {
    pub fn efficacy(&self, damage: Type, target: Type) -> Efficacy {
        return self.table[EfficacyTable::index(damage, target)];
    }

    pub fn index(damage: Type, target: Type) -> usize {
        ((damage.repr() as usize) * TYPE_COUNT) + (target.repr() as usize)
    }
}

impl vcsv::FromCsv for EfficacyTable {
    fn from_csv<'e, R: std::io::Read>(
        reader: &mut csv::Reader<R>
    ) -> vcsv::Result<'e, Self> {
        let mut table = EfficacyTable {
            table: [Efficacy::Regular; TYPE_COUNT*TYPE_COUNT],
        };
        for result in reader.records() {
            let record = result?;
            let damage = vcsv::from_field(&record, 0)?;
            let target = vcsv::from_field(&record, 1)?;
            let efficacy = vcsv::from_field(&record, 2)?;
            table.table[EfficacyTable::index(damage, target)] = efficacy;
        }
        Ok(table)
    }
}
