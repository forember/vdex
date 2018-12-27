//! Items and related metadata.

pub(self) mod berries;
pub(self) mod flags;

pub use self::berries::Berry;
pub use self::berries::BERRY_COUNT;
pub use self::berries::Flavor;
pub use self::flags::Flags;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use enums::*;
use FromVeekun;
use to_pascal_case;
use vcsv;
use vcsv::FromCsv;
use VeekunOption;

/// Broad item category; not used for anything other than organization.
#[EnumRepr(type = "u8")]
pub enum Category {
    /// X *Stat*, Dire Hit, and Guard Spec.
    StatBoosts = 1,
    /// Berries that lower EVs and raise happiness; unused in pbirch.
    EffortDrop,
    /// Berries that act as medicine.
    Medicine,
    /// Miscellaneous berries.
    Other,
    /// Berries consumed at quarter HP, generally to boost a stat.
    InAPinch,
    /// Berries that heal 1/8 HP if their flavor is not disliked.
    PickyHealing,
    /// Berries that halve damage of a typed attack, usually only when super
    /// effective.
    TypeProtection,
    /// Berries that are only useful for baking; unused in pbirch.
    BakingOnly,
    /// Items that have no effect, but can be traded for items or moves; unused
    /// in pbirch.
    Collectibles,
    /// Items involved in evolution.
    Evolution,
    /// Non-held items that affect wild battles, and the Escape Rope; unused in
    /// pbirch.
    Spelunking,
    /// Miscellaneous held items.
    HeldItems,
    /// Choice Band, Scarf, and Specs.
    Choice,
    /// Items that add EVs, but halve Speed, and the Macho Brace; unused in
    /// pbirch.
    EffortTraining,
    /// Held items that have a negative effect on the holder.
    BadHeldItems,
    /// Various held items useful in training; unused in pbirch.
    Training,
    /// Arceus type plates.
    Plates,
    /// Held items that only affect a specific species.
    SpeciesSpecific,
    /// Held items that increase the damage of typed moves.
    TypeEnhancement,
    /// Key items from Nintendo events; unused in pbirch.
    EventItems,
    /// Key items to facilitate various gameplay elements; unused in pbirch.
    Gameplay,
    /// Key items to facilitate plot advancement; unused in pbirch.
    PlotAdvancement,
    /// Key items that have code but are unused; unused in pbirch.
    Unused,
    /// Valuables that can be sold or traded; unused in pbirch.
    Loot,
    /// Held items which may contain a message for a trade; unused in pbirch.
    Mail,
    /// Medicines which increase EVs; unused in pbirch.
    Vitamins,
    /// Medicines which restore HP.
    Healing,
    /// Medicines which restore PP.
    PPRecovery,
    /// Medicines which revive Pokémon from fainting.
    Revival,
    /// Medicines which cure status ailments.
    StatusCures,
    /// Items to be used on soil to affect berry growth; unused in pbirch.
    Mulch = 32,
    /// Poké Balls which have a special effect; unused in pbirch.
    SpecialBalls,
    /// Poké Balls without any special effect; unused in pbirch.
    StandardBalls,
    /// Fossils, Honey, and the Odd Keystone.
    DexCompletion,
    /// Held items which raise the holder's contest condition; unused in pbirch.
    Scarves,
    /// TMs and HMs.
    Machines,
    /// Blue, Red, and Yellow Flutes.
    Flutes,
    /// Poké Balls produced from apricorns; unused in pbirch.
    ApricornBalls,
    /// Apricorns; unused in pbirch.
    ApricornBox,
    /// Key items which record Pokéathlon statistics; unused in pbirch.
    DataCards,
    /// Held items which are consumed, increasing the power of a typed move.
    Jewels,
    /// Wonder Launcher items; unused in pbirch.
    MiracleShooter,
}

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

/// Bag pocket in which items are stored.
#[EnumRepr(type = "u8")]
pub enum Pocket {
    Misc = 1,
    Medicine,
    Pokeballs,
    Machines,
    Berries,
    Mail,
    Battle,
    Key,
}

impl Category {
    /// True if the items in this category have no use in the pbirch simulation.
    pub fn unused(self) -> bool {
        match self.repr() {
            2 | 8 | 9 | 11 | 14 | 16 | 20 ... 26 | 32 ... 34 | 36 | 39 ... 41
                | 43  => true,
            _ => false,
        }
    }

    /// Get the bag pocket in which items of this category are stored.
    pub fn pocket(self) -> Pocket {
        match self.repr() {
            9 ... 19 | 24 | 32 | 35 | 36 | 42 => Pocket::Misc,
            26 ... 30 => Pocket::Medicine,
            33 | 34 | 39 => Pocket::Pokeballs,
            37 => Pocket::Machines,
            2 ... 8 => Pocket::Berries,
            25 => Pocket::Mail,
            1 | 38 | 43 => Pocket::Battle,
            20 ... 23 | 40 | 41 => Pocket::Key,
            _ => unreachable!(),
        }
    }
}

impl FromVeekun for Category {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Category::from_repr(value)
    }
}

impl FromVeekun for FlingEffect {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        FlingEffect::from_repr(value)
    }
}

/// A bag item.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Item) An
/// > item (Japanese: 道具 tool) is an object in the Pokémon games which the
/// > player can pick up, keep in their Bag, and use in some manner. They have
/// > various uses, including healing, powering up, helping one to catch
/// > Pokémon, or to access a new area.
#[derive(Debug)]
pub struct Item {
    /// The Veekun ID for the item.
    pub id: u16,
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
pub struct ItemTable(pub HashMap<u16, Item>);

impl ItemTable {
    /// Create an item table from the provided CSV files.
    pub fn from_files<S: AsRef<OsStr> + ?Sized>(
        items_file: &S, flags_file: &S, berries_file: &S, flavors_file: &S
    ) -> vcsv::Result<Self> {
        let berries_table
            = berries::BerryTable::from_files(berries_file, flavors_file)?;
        let flags_path = Path::new(flags_file);
        let flags_table = flags::FlagTable::from_csv_file(flags_path)?;
        let items_path = Path::new(items_file);
        let mut items_table = ItemTable::from_csv_file(items_path)?;
        items_table.set_flags(&flags_table);
        items_table.set_berries(&berries_table);
        Ok(items_table)
    }

    fn set_flags(&mut self, flag_table: &flags::FlagTable) {
        for (id, item) in self.0.iter_mut() {
            item.flags = flag_table.0.get(id)
                .map_or(flags::Flags::empty(), |v| *v);
        }
    }

    fn set_berries(&mut self, berry_table: &berries::BerryTable) {
        for berry in berry_table.0.iter() {
            if let Some(item) = self.0.get_mut(&berry.item_id) {
                item.berry = Some(*berry);
            }
        }
    }
}

impl std::ops::Index<u16> for ItemTable {
    type Output = Item;

    fn index<'a>(&'a self, index: u16) -> &'a Item {
        self.0.index(&index)
    }
}

impl vcsv::FromCsvIncremental for ItemTable {
    fn from_empty_csv() -> Self {
        ItemTable(HashMap::new())
    }

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
