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
use veekun::repr::VeekunOption;

#[EnumRepr(type = "u8")]
pub enum Category {
    StatBoosts = 1,
    EffortDrop,
    Medicine,
    Other,
    InAPinch,
    PickyHealing,
    TypeProtection,
    BakingOnly,
    Collectibles,
    Evolution,
    Spelunking,
    HeldItems,
    Choice,
    EffortTraining,
    BadHeldItems,
    Training,
    Plates,
    SpeciesSpecific,
    TypeEnhancement,
    EventItems,
    Gameplay,
    PlotAdvancement,
    Unused,
    Loot,
    Mail,
    Vitamins,
    Healing,
    PPRecovery,
    Revival,
    StatusCures,
    Mulch = 32,
    SpecialBalls,
    StandardBalls,
    DexCompletion,
    Scarves,
    Machines,
    Flutes,
    ApricornBalls,
    ApricornBox,
    DataCards,
    Jewels,
    MiracleShooter,
}

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

#[derive(Debug)]
pub struct Item {
    pub id: u16,
    pub name: String,
    pub category: Category,
    pub cost: u16,
    pub fling_power: Option<u8>,
    pub fling_effect: FlingEffect,
    pub flags: Flags,
    pub berry: Option<Berry>,
}

pub struct ItemTable(pub HashMap<u16, Item>);

impl ItemTable {
    pub fn from_files<'e, S: AsRef<OsStr> + ?Sized>(
        items_file: &S, flags_file: &S, berries_file: &S, flavors_file: &S
    ) -> vcsv::Result<'e, Self> {
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

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
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
