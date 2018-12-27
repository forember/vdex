//! Traits for loading Pokédex tables from Veekun CSV files.

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::path::Path;
use repr::FromVeekunField;

/// Miscellaneous error intended for `Error::Veekun`. Just wraps a string
/// literal.
#[derive(Debug)]
pub struct MiscError(&'static str);

impl From<&'static str> for MiscError {
    fn from(s: &'static str) -> Self {
        MiscError(s)
    }
}

impl Display for MiscError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for MiscError { }

/// Error in a Veekun CSV file.
#[derive(Debug)]
pub enum Error {
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
        /// Error object (usually of type `veekun::repr::Error`).
        error: Box<StdError>,
    },
}

impl Error {
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

impl From<csv::Error> for Error {
    fn from(error: csv::Error) -> Self {
        Error::Csv(error)
    }
}

impl Display for Error {
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
            Error::Veekun { line, field, error } => {
                let line_str = line
                    .map_or("?".to_string(), |n| format!("{}", n));
                write!(f, "Error on line {} field {}: {}",
                       line_str, field, error)
            },
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Csv(error) => Some(error),
            Error::Veekun { error, .. } => Some(error.as_ref()),
            _ => None,
        }
    }
}

/// The type returned by Veekun CSV functions.
pub type Result<T> = std::result::Result<T, Error>;

/// Get the line number of a record, if it is available.
pub fn get_line(record: &csv::StringRecord) -> Option<u64> {
    record.position().map(csv::Position::line)
}

/// Get the string for a field, or an Error on out-of-bounds.
pub fn get_field(
    record: &csv::StringRecord, index: usize
) -> Result<&str> {
    record.get(index).ok_or_else(|| Error::RecordLength {
        line: get_line(record),
        index
    })
}

/// Read a value from a CSV field. See `from_field` and `from_option_field`.
pub fn from_veekun_field<T: FromVeekunField>(
    line: Option<u64>, index: usize, field: &str, default: Option<T>
) -> Result<T>
    where <T as FromVeekunField>::VeekunErr: 'static + StdError
{
    T::from_veekun_field(field, default).or_else(|e| Err(Error::Veekun {
        line,
        field: index,
        error: Box::new(e),
    }))
}

/// Like `from_field`, but with a default.
///
/// See `veekun::FromVeekunField::from_veekun_field` for details.
pub fn from_option_field<T: FromVeekunField>(
    record: &csv::StringRecord, index: usize, default: T
) -> Result<T>
    where <T as FromVeekunField>::VeekunErr: 'static + StdError
{
    let field = get_field(record, index)?;
    from_veekun_field(get_line(record), index, field, Some(default))
}

/// Read a value from a CSV field. Useful for implementing `FromCsv`.
pub fn from_field<T: FromVeekunField>(
    record: &csv::StringRecord, index: usize
) -> Result<T>
    where <T as FromVeekunField>::VeekunErr: 'static + StdError
{
    let field = get_field(record, index)?;
    from_veekun_field(get_line(record), index, field, None)
}

/// Abstracts creating an object by loading a CSV file.
pub trait FromCsv: Sized {
    /// Creates a `Reader` from the path and passes it to `from_csv`.
    fn from_csv_file(path: &Path) -> Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        Self::from_csv(&mut reader)
    }

    /// Loads the object from an open CSV file.
    fn from_csv<R: Read>(reader: &mut csv::Reader<R>) -> Result<Self>;
}

/// Convenience trait for implementing `FromCsv` where each record is loaded
/// individually.
pub trait FromCsvIncremental: Sized { 
    /// Create the initial state of the object.
    fn from_empty_csv() -> Self;

    /// Update the object from a record.
    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> Result<()>;
}

impl<T: FromCsvIncremental> FromCsv for T {
    fn from_csv<R: Read>(reader: &mut csv::Reader<R>) -> Result<T> {
        let mut state = T::from_empty_csv();
        for result in reader.records() {
            let record = result?;
            state.load_csv_record(record)?;
        }
        Ok(state)
    }
}
