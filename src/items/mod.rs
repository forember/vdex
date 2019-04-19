//! Items and related data.

pub(self) mod bag;
pub(self) mod berries;
pub(self) mod flags;

pub use self::bag::Category;
pub use self::bag::Pocket;
pub use self::berries::Berry;
pub use self::berries::BERRY_COUNT;
pub use self::berries::Flavor;
pub use self::flags::Flags;

use std::collections::HashMap;
use crate::enums::*;
use crate::FromVeekun;
use crate::to_pascal_case;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use crate::VeekunOption;

/// Extra effect when thrown using Fling.
#[EnumRepr(type = "u8")]
pub enum FlingEffect {
    None = 0,
    BadlyPoison,
    Burn,
    ActivateBerry,
    ActivateHerb,
    Paralyze,
    Poison,
    Flinch,
}

impl Default for FlingEffect {
    fn default() -> Self { FlingEffect::None }
}

impl FromVeekun for FlingEffect {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        FlingEffect::from_repr(value)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ItemId(pub u16);

impl Default for ItemId {
    fn default() -> Self { ItemId(0) }
}

impl FromVeekun for ItemId {
    type Intermediate = u16;

    fn from_veekun(value: u16) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(ItemId(value))
        }
    }
}

/// A bag item.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Item) An
/// > item (Japanese: 道具 tool) is an object in the Pokémon games which the
/// > player can pick up, keep in their Bag, and use in some manner. They have
/// > various uses, including healing, powering up, helping one to catch
/// > Pokémon, or to access a new area.
#[derive(Clone, Debug, Default)]
pub struct Item {
    /// The pbirch id for the item.
    pub id: ItemId,
    /// The pbirch name for the item.
    pub name: String,
    /// The item's category.
    ///
    /// The category can be used to derive the unused and pocket properties.
    pub category: Category,
    /// The cost to buy the item.
    pub cost: u16,
    /// The power of Fling with this item, or `None` if it cannot be flung.
    pub fling_power: Option<u8>,
    /// The effect of Fling with this item.
    pub fling_effect: FlingEffect,
    /// Item bitflags.
    pub flags: Flags,
    /// Berry properties, or `None` if the item is not a berry.
    pub berry: Option<Berry>,
}

/// Wrapper of a `HashMap` mapping IDs to items.
///
/// Use `table.0` to access `HashMap` members.
#[derive(Default)]
pub struct ItemTable(pub HashMap<ItemId, Item>);

impl ItemTable {
    /// Create an item table from the included CSV data.
    pub fn new() -> Self {
        let mut items_table = ItemTable::from_csv_data(vdata::ITEMS).unwrap();
        items_table.set_berries(&berries::BerryTable::new());
        items_table.set_flags(&flags::FlagTable::new());
        items_table
    }

    fn set_berries(&mut self, berry_table: &berries::BerryTable) {
        for berry in berry_table.0.iter() {
            if let Some(item) = self.0.get_mut(&berry.item) {
                item.berry = Some(*berry);
            }
        }
    }

    fn set_flags(&mut self, flag_table: &flags::FlagTable) {
        for (id, item) in self.0.iter_mut() {
            item.flags = flag_table.0.get(id)
                .map_or(flags::Flags::empty(), |v| *v);
        }
    }
}

impl vcsv::FromCsvIncremental for ItemTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        let fling_power: VeekunOption<_> = vcsv::from_field(&record, 4)?;
        self.0.insert(id, Item {
            id,
            name: to_pascal_case(vcsv::get_field(&record, 1)?),
            category: vcsv::from_field(&record, 2)?,
            cost: vcsv::from_field(&record, 3)?,
            fling_power: fling_power.into(),
            fling_effect:
                vcsv::from_option_field(&record, 5, FlingEffect::None)?,
            flags: flags::Flags::empty(),
            berry: None,
        });
        Ok(())
    }
}

impl std::ops::Index<ItemId> for ItemTable {
    type Output = Item;

    fn index(&self, index: ItemId) -> &Item {
        self.0.index(&index)
    }
}
