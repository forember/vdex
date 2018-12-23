#[macro_use]
extern crate bitflags;
extern crate enum_repr;
extern crate veekun;

pub(self) mod abilities;
pub(self) mod enums;
pub mod items;
pub mod moves;
pub(self) mod natures;
pub mod pokemon;
pub(self) mod types;
pub mod versions;

pub use enums::Enum;
pub use abilities::Ability;
pub use natures::*;
pub use types::*;

use veekun::csv as vcsv;
use veekun::to_pascal_case;
use veekun::repr::FromVeekun;

#[cfg(test)]
mod tests;
