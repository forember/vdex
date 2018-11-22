pub mod csv;

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error<V>
    where V: FromStr + Debug, <V as FromStr>::Err: Debug
{
    Value(V),
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

pub trait FromVeekun<V>: Sized
    where V: FromStr + Debug + Copy, <V as FromStr>::Err: Debug
{
    fn from_veekun(value: V) -> Option<Self>;

    fn from_veekun_field(field: &str) -> Result<Self, Error<V>> {
        let value = field.parse().or_else(|e| Err(Error::Parse(e)))?;
        Self::from_veekun(value).ok_or(Error::Value(value))
    }
}

impl<V> FromVeekun<V> for V
    where V: FromStr + Debug + Copy, <V as FromStr>::Err: Debug
{
    fn from_veekun(value: V) -> Option<Self> {
        Some(value)
    }
}
