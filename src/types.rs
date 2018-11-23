use enums::*;
use veekun;

enum_repr!("i8";
pub enum Efficacy {
    Not = -2,
    NotVery,
    Regular,
    Super,
});

enum_repr!("u8";
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
});

pub fn assert_sanity() {
    assert_eq!(Efficacy::Super.repr(), 1);
    assert_eq!(Type::Dark.repr(), 16);
}

impl veekun::FromVeekun<u8> for Efficacy {
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

impl veekun::FromVeekun<u8> for Type {
    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Type::from_repr)
    }
}

pub struct EfficacyTable {
    pub table: [Efficacy; 17*17],
}

impl EfficacyTable {
    pub fn efficacy(&self, damage: Type, target: Type) -> Efficacy {
        return self.table[EfficacyTable::index(damage, target)];
    }

    pub fn index(damage: Type, target: Type) -> usize {
        ((damage.repr() as usize) * 17) + (target.repr() as usize)
    }
}

impl veekun::csv::FromCsv for EfficacyTable {
    fn from_csv<'e, R: std::io::Read>(
        reader: &mut csv::Reader<R>
    ) -> veekun::csv::Result<'e, Self> {
        let mut table = EfficacyTable {
            table: [Efficacy::Regular; 17*17],
        };
        for result in reader.records() {
            let record = result?;
            let damage = veekun::csv::from_field(&record, 0)?;
            let target = veekun::csv::from_field(&record, 1)?;
            let efficacy = veekun::csv::from_field(&record, 2)?;
            table.table[EfficacyTable::index(damage, target)] = efficacy;
        }
        Ok(table)
    }
}
