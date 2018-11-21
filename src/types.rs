use enum_repr::EnumRepr;
use std::error;
use std::fmt;
use std::num;

#[EnumRepr(type = "i8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Efficacy {
    Not = -2,
    NotVery,
    Regular,
    Super,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

#[derive(Debug)]
pub enum Error {
    CsvError(csv::Error),
    ParseIntError(num::ParseIntError),
    InvalidEfficacy(u8),
    InvalidType(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CsvError(e)
                => write!(f, "{}", e),
            Error::ParseIntError(e)
                => write!(f, "{}", e),
            Error::InvalidEfficacy(x)
                => write!(f, "Invalid efficacy: {}", x),
            Error::InvalidType(x)
                => write!(f, "Invalid Pokemon type: {}", x),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::CsvError(e) => Some(e),
            Error::ParseIntError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::CsvError(error)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error::ParseIntError(error)
    }
}

pub struct EfficacyTable {
    table: [Efficacy; 17*17],
}

pub type Result<T> = std::result::Result<T, Error>;

impl EfficacyTable {
    pub fn efficacy(&self, damage: Type, target: Type) -> Efficacy {
        return self.table[EfficacyTable::index(damage, target)];
    }

    fn index(damage: Type, target: Type) -> usize {
        ((damage.repr() as usize) * 17) + (target.repr() as usize)
    }

    fn parse_type(field: &str) -> Result<Type> {
        let x = field.parse()?;
        Type::from_repr(x - 1).ok_or(Error::InvalidType(x))
    }

    fn parse_efficacy(field: &str) -> Result<Efficacy> {
        let x = field.parse()?;
        match x {
            0 => Ok(Efficacy::Not),
            50 => Ok(Efficacy::NotVery),
            100 => Ok(Efficacy::Regular),
            200 => Ok(Efficacy::Super),
            _ => Err(Error::InvalidEfficacy(x)),
        }
    }

    pub fn from_csv_file(path: &std::path::Path) -> Result<EfficacyTable> {
        let mut table = EfficacyTable {
            table: [Efficacy::Regular; 17*17],
        };
        let mut reader = csv::Reader::from_path(path)?;
        for result in reader.records() {
            let record = result?;
            let damage = EfficacyTable::parse_type(&record[0])?;
            let target = EfficacyTable::parse_type(&record[1])?;
            let efficacy = EfficacyTable::parse_efficacy(&record[2])?;
            table.table[EfficacyTable::index(damage, target)] = efficacy;
        }
        Ok(table)
    }
}

pub fn assert_sanity() {
    assert_eq!(Efficacy::Super.repr(), 1);
    assert_eq!(Type::Dark.repr(), 16);
}
