use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use veekun;

#[derive(Debug)]
pub enum Error {
    Veekun {
        line: u64,
        field: usize,
        debug: Box<Debug>,
    },
    RecordLength {
        line: u64,
    },
    Csv(csv::Error),
}

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Veekun { line, field, debug } => {
                write!(f, "Error on line {} field {}: {:?}", line, field, debug)
            },
            Error::RecordLength { line } => {
                write!(f, "Record on line {} too short.", line)
            },
            Error::Csv(error) => {
                write!(f, "{}", error)
            },
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Csv(error) => Some(error),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn from_field<V, T: veekun::FromVeekun<V>>(
    record: &csv::StringRecord, index: usize
) -> Result<T>
    where V: FromStr + Debug + 'static, <V as FromStr>::Err: Debug
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
    fn from_csv_file(path: &Path) -> Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        Self::from_csv(&mut reader)
    }

    fn from_csv<R: Read>(reader: &mut csv::Reader<R>) -> Result<Self>;
}
