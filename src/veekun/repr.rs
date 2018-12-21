//! The `FromVeekun` trait allows for conversion from the representations in
//! the Veekun CSV files to pbirch types.

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Abstracts the idea of creating a new instance from a CSV field.
pub trait FromVeekunField: Sized {
    type VeekunErr;

    /// Parses the CSV field.
    ///
    /// The only effect the value of `default` may have on the return value is
    /// to return Ok( *x* ) where `default` is Some( *x* ), and an error whould
    /// have been returned for the given field argument had `default` been None.
    ///
    /// Otherwise, the conditions under which `default` is returned are up to
    /// the implementation, but typically it is returned when the field is empty
    /// or whitespace (and, of course, the above conditions are met). For
    /// default-on-error behavior, pass `None` to `default` and call `or` on the
    /// result.
    fn from_veekun_field(
        field: &str, default: Option<Self>
    ) -> Result<Self, Self::VeekunErr>;
}

pub trait FromVeekun: Sized {
    type Intermediate;

    /// Creates a new instance from the parsed CSV field value.
    fn from_veekun(value: Self::Intermediate) -> Option<Self>;
}

/// Blanket implementation for parsing `FromStr` types directly from Veekun CSV
/// files.
impl<T> FromVeekun for T
    where T: FromStr + Debug + Copy, <T as FromStr>::Err: Debug
{
    type Intermediate = T;

    fn from_veekun(value: T) -> Option<Self> { Some(value) }
}

/// An error in the Veekun CSV representation.
#[derive(Debug)]
pub enum VeekunError<V>
    where V: FromStr + Debug, <V as FromStr>::Err: Debug
{
    /// The parsed value was not valid.
    Value(V),
    /// The CSV field could not be parsed.
    Parse(V::Err),
}

impl<V> Display for VeekunError<V>
    where V: FromStr + Debug + Display, <V as FromStr>::Err: Debug + Display
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            VeekunError::Value(v)
                => write!(f, "Invalid value: {}", v),
            VeekunError::Parse(e)
                => write!(f, "{}", e),
        }
    }
}

impl<V> StdError for VeekunError<V>
    where V: FromStr + Debug + Display,
        <V as FromStr>::Err: Debug + StdError + 'static
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            VeekunError::Value(_) => None,
            VeekunError::Parse(e) => Some(e),
        }
    }
}

/// Blanket implementation for `FromVeekun` types.
impl<T> FromVeekunField for T
    where T: FromVeekun,
          <T as FromVeekun>::Intermediate: FromStr + Debug + Copy,
          <<T as FromVeekun>::Intermediate as FromStr>::Err: Debug
{
    type VeekunErr = VeekunError<T::Intermediate>;

    /// Parses the field string and passes the value to `from_veekun`.
    fn from_veekun_field(
        field: &str, default: Option<Self>
    ) -> Result<Self, Self::VeekunErr> {
        let result = field.parse();
        if let Err(e) = result {
            let error = VeekunError::Parse(e);
            if let Some(d) = default {
                if field.chars().all(char::is_whitespace) {
                    return Ok(d)
                }
            }
            return Err(error)
        }
        let value = result.unwrap();
        Self::from_veekun(value).ok_or(VeekunError::Value(value))
    }
}

pub struct VeekunOption<T>(Option<T>);

impl<T> Into<Option<T>> for VeekunOption<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

impl<T: FromVeekunField> FromVeekunField for VeekunOption<T> {
    type VeekunErr = <T as FromVeekunField>::VeekunErr;

    fn from_veekun_field(
        field: &str, default: Option<Self>
    ) -> Result<Self, Self::VeekunErr> {
        if field.chars().all(char::is_whitespace) {
            Ok(VeekunOption(None))
        } else {
            T::from_veekun_field(field, default.and_then(|v| v.into()))
                .map(|v| VeekunOption(Some(v)))
        }
    }
}
