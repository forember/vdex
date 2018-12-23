//! The `FromCsv` trait allows for loading pbirch Pokédex tables from
//! Veekun CSV files.

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io::Read;
use std::path::Path;
use repr::FromVeekunField;

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
                    .map_or("?".to_string(), |n| format!("{}", n));
                write!(f, "Record on line {} too short for field index {}.",
                       line_str, index)
            },
            Error::Veekun { line, field, debug } => {
                let line_str = line
                    .map_or("?".to_string(), |n| format!("{}", n));
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

pub fn get_line(record: &csv::StringRecord) -> Option<u64> {
    record.position().map(csv::Position::line)
}

pub fn get_field<'e>(
    record: &csv::StringRecord, index: usize
) -> Result<'e, &str> {
    record.get(index).ok_or_else(|| Error::RecordLength {
        line: get_line(record),
        index
    })
}

pub fn from_veekun_field<'e, T: FromVeekunField>(
    line: Option<u64>, index: usize, field: &str, default: Option<T>
) -> Result<'e, T>
    where <T as FromVeekunField>::VeekunErr: 'e + Debug
{
    T::from_veekun_field(field, default).or_else(|e| Err(Error::Veekun {
        line,
        field: index,
        debug: Box::new(e),
    }))
}

pub fn from_option_field<'e, T: FromVeekunField>(
    record: &csv::StringRecord, index: usize, default: T
) -> Result<'e, T>
    where <T as FromVeekunField>::VeekunErr: 'e + Debug
{
    let field = get_field(record, index)?;
    from_veekun_field(get_line(record), index, field, Some(default))
}

/// Read a value from a CSV field. Useful for implementing `FromCsv`.
pub fn from_field<'e, T: FromVeekunField>(
    record: &csv::StringRecord, index: usize
) -> Result<'e, T>
    where <T as FromVeekunField>::VeekunErr: 'e + Debug
{
    let field = get_field(record, index)?;
    from_veekun_field(get_line(record), index, field, None)
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
