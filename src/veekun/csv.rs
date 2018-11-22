use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use veekun;

#[derive(Debug)]
pub enum Error<'e> {
    Veekun {
        line: u64,
        field: usize,
        debug: Box<Debug + 'e>,
    },
    Csv(csv::Error),
    RecordLength {
        line: u64,
    },
}

impl<'e> From<csv::Error> for Error<'e> {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error)
    }
}

impl<'e> Display for Error<'e> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Veekun { line, field, debug } => {
                write!(f, "Error on line {} field {}: {:?}", line, field, debug)
            },
            Error::Csv(error) => {
                write!(f, "{}", error)
            },
            Error::RecordLength { line } => {
                write!(f, "Record on line {} too short.", line)
            },
        }
    }
}

impl<'e> StdError for Error<'e> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Csv(error) => Some(error),
            _ => None,
        }
    }
}

pub type Result<'e, T> = std::result::Result<T, Error<'e>>;

pub fn from_field<'e, V, T: veekun::FromVeekun<V>>(
    record: &csv::StringRecord, index: usize
) -> Result<'e, T>
    where V: FromStr + Debug + Copy + 'e, <V as FromStr>::Err: Debug
{
    let line = match record.position() {
        Some(p) => p.line(),
        None => 0,
    };
    let field = record.get(index).ok_or(Error::RecordLength { line })?;
    T::from_veekun_field(field).or_else(|e| Err(Error::Veekun {
        line: line,
        field: index,
        debug: Box::new(e),
    }))
}

pub trait FromCsv: Sized {
    fn from_csv_file<'e>(path: &Path) -> Result<'e, Self> {
        let mut reader = csv::Reader::from_path(path)?;
        Self::from_csv(&mut reader)
    }

    fn from_csv<'e, R: Read>(reader: &mut csv::Reader<R>) -> Result<'e, Self>;
}
