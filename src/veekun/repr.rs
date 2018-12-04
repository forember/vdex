//! The `FromVeekun` trait allows for conversion from the representations in
//! the Veekun CSV files to pbirch types.

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// An error in the Veekun CSV representation.
#[derive(Debug)]
pub enum Error<V>
    where V: FromStr + Debug, <V as FromStr>::Err: Debug
{
    /// The parsed value was not valid.
    Value(V),
    /// The CSV field could not be parsed.
    Parse(V::Err),
}

impl<V> Display for Error<V>
    where V: FromStr + Debug + Display, <V as FromStr>::Err: Debug + Display
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Value(v)
                => write!(f, "Invalid value: {}", v),
            Error::Parse(e)
                => write!(f, "{}", e),
        }
    }
}

impl<V> StdError for Error<V>
    where V: FromStr + Debug + Display,
        <V as FromStr>::Err: Debug + StdError + 'static
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Value(_) => None,
            Error::Parse(e) => Some(e),
        }
    }
}

/// Abstracts the idea of creating a new instance from a CSV field.
pub trait FromVeekun<V>: Sized
    where V: FromStr + Debug + Copy, <V as FromStr>::Err: Debug
{
    /// Creates a new instance from the parsed CSV field value.
    fn from_veekun(value: V) -> Option<Self>;

    /// Parses the field string and passes the value to `from_veekun`.
    fn from_veekun_field(field: &str) -> Result<Self, Error<V>> {
        let value = field.parse().or_else(|e| Err(Error::Parse(e)))?;
        Self::from_veekun(value).ok_or(Error::Value(value))
    }
}

/// Blanket implementation for parsing `FromStr` types directly from Veekun
/// CSV files.
impl<V> FromVeekun<V> for V
    where V: FromStr + Debug + Copy, <V as FromStr>::Err: Debug
{
    fn from_veekun(value: V) -> Option<Self> {
        Some(value)
    }
}

