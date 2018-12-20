//! The `FromCsv` trait allows for loading pbirch Pokédex tables from
//! Veekun CSV files.

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use FromVeekun;

/// Error in a Veekun CSV file. The lifetime `'e` is the lifetime of the boxed
/// Veekun representation error.
#[derive(Debug)]
pub enum Error<'e> {
    /// CSV format error.
    Csv(csv::Error),
    /// Record too short.
    RecordLength {
        line: Option<u64>,
        /// Attempted out-of-bounds index.
        index: usize,
    },
    /// Representation error.
    Veekun {
        line: Option<u64>,
        /// Field number on the line.
        field: usize,
        /// Object for debug output (usually of type `veekun::repr::Error`).
        debug: Box<Debug + 'e>,
    },
}

impl<'e> Error<'e> {
    /// Line number on which the error occurred, if it is available.
    pub fn line(&self) -> Option<u64> {
        match self {
            Error::Csv(e) => match e.kind() {
                csv::ErrorKind::Utf8 { pos, .. } => pos.clone(),
                csv::ErrorKind::UnequalLengths { pos, .. } => pos.clone(),
                csv::ErrorKind::Deserialize { pos, .. } => pos.clone(),
                _ => None,
            }.and_then(|p| Some(p.line())),
            Error::RecordLength { line, .. } => *line,
            Error::Veekun { line, .. } => *line,
        }
    }
}

impl<'e> From<csv::Error> for Error<'e> {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error)
    }
}

impl<'e> Display for Error<'e> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Csv(error) => {
                write!(f, "{}", error)
            },
            Error::RecordLength { line, index } => {
                let line_str = line
                    .and_then(|n| Some(format!("{}", n)))
                    .unwrap_or("?".to_string());
                write!(f, "Record on line {} too short for field index {}.",
                       line_str, index)
            },
            Error::Veekun { line, field, debug } => {
                let line_str = line
                    .and_then(|n| Some(format!("{}", n)))
                    .unwrap_or("?".to_string());
                write!(f, "Error on line {} field {}: {:?}",
                       line_str, field, debug)
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

/// Read a value from a CSV field. Useful for implementing `FromCsv`.
pub fn from_field<'e, V, T: FromVeekun<V>>(
    record: &csv::StringRecord, index: usize
) -> Result<'e, T>
    where V: FromStr + Debug + Copy + 'e, <V as FromStr>::Err: Debug
{
    let line = record.position().and_then(|p| Some(p.line()));
    let field = record.get(index).ok_or(Error::RecordLength { line, index })?;
    T::from_veekun_field(field).or_else(|e| Err(Error::Veekun {
        line: line,
        field: index,
        debug: Box::new(e),
    }))
}

/// Abstracts creating an object by loading a CSV file.
pub trait FromCsv: Sized {
    /// Creates a `Reader` from the path and passes it to `from_csv`.
    fn from_csv_file<'e>(path: &Path) -> Result<'e, Self> {
        let mut reader = csv::Reader::from_path(path)?;
        Self::from_csv(&mut reader)
    }

    /// Loads the object from an open CSV file.
    fn from_csv<'e, R: Read>(reader: &mut csv::Reader<R>) -> Result<'e, Self>;
}

pub trait FromCsvIncremental: Sized { 
    fn from_empty_csv() -> Self;

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> Result<'e, ()>;
}

impl<T: FromCsvIncremental> FromCsv for T {
    fn from_csv<'e, R: Read>(reader: &mut csv::Reader<R>) -> Result<'e, T> {
        let mut state = T::from_empty_csv();
        for result in reader.records() {
            let record = result?;
            state.load_csv_record(record)?;
        }
        Ok(state)
    }
}
