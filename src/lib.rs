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

pub use abilities::Ability;
pub use enums::Enum;
pub use natures::*;
pub use types::*;

use veekun::csv as vcsv;
use veekun::data as vdata;
use veekun::repr::{FromVeekun, VeekunOption};
use veekun::to_pascal_case;

#[cfg(test)]
mod tests;

pub struct Pokedex {
    pub efficacy: EfficacyTable,
    pub items: items::ItemTable,
    pub moves: moves::MoveTable,
    pub palace: PalaceTable,
    pub species: pokemon::SpeciesTable,
}
