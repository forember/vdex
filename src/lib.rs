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

pub use self::abilities::Ability;
pub use self::enums::Enum;
pub use self::natures::*;
pub use self::types::*;

use veekun::csv as vcsv;
use veekun::data as vdata;
use veekun::repr::{FromVeekun, VeekunOption};
use veekun::to_pascal_case;

#[cfg(test)]
mod tests;

/// All the data in vdex.
pub struct Pokedex {
    pub efficacy: EfficacyTable,
    pub items: items::ItemTable,
    pub moves: moves::MoveTable,
    pub palace: PalaceTable,
    pub species: pokemon::SpeciesTable,
}

impl Pokedex {
    pub fn new() -> Self {
        Pokedex {
            efficacy: EfficacyTable::new(),
            items: items::ItemTable::new(),
            moves: moves::MoveTable::new(),
            palace: PalaceTable::new(),
            species: pokemon::SpeciesTable::new(),
        }
    }
}

static mut POKEDEX: Option<Pokedex> = None;
static POKEDEX_ONCE: std::sync::Once = std::sync::ONCE_INIT;

/// START HERE: Load (if not loaded) and return the global Pokedex instance.
pub fn pokedex() -> &'static Pokedex {
    unsafe {
        POKEDEX_ONCE.call_once(|| {
            POKEDEX = Some(Pokedex::new());
        });
        POKEDEX.as_ref().unwrap()
    }
}
