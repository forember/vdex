pub mod berries;
pub mod flags;

use std::collections::HashMap;
use enums::*;
use FromVeekun;
use vcsv;
use veekun;
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

pub fn assert_sanity() {
    assert_eq!(Category::StatusCures.repr(), 30);
    assert_eq!(Category::MiracleShooter.repr(), 43);
    assert_eq!(FlingEffect::Flinch.repr(), 7);
    assert_eq!(Pocket::Key.repr(), 8);
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
pub struct Item<'a> {
    pub id: u16,
    pub name: String,
    pub category: Category,
    pub cost: u16,
    pub fling_power: Option<u8>,
    pub fling_effect: FlingEffect,
    pub flags: flags::Flags,
    pub berry: Option<&'a berries::Berry>,
}

pub struct ItemTable<'a>(pub HashMap<u16, Item<'a>>);

impl<'a> ItemTable<'a> {
    pub fn set_flags(&mut self, flag_table: &flags::FlagTable) {
        for (id, item) in self.0.iter_mut() {
            item.flags = flag_table.0.get(id)
                .map_or(flags::Flags::empty(), |v| *v);
        }
    }

    pub fn link_berries(&mut self, berry_table: &'a berries::BerryTable) {
        for berry in berry_table.0.iter() {
            if let Some(item) = self.0.get_mut(&berry.item_id) {
                item.berry = Some(berry);
            }
        }
    }
}

impl<'a> std::ops::Index<u16> for ItemTable<'a> {
    type Output = Item<'a>;

    fn index<'b>(&'b self, index: u16) -> &'b Item<'a> {
        self.0.index(&index)
    }
}

impl<'a> vcsv::FromCsvIncremental for ItemTable<'a> {
    fn from_empty_csv() -> Self {
        ItemTable(HashMap::new())
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id = vcsv::from_field(&record, 0)?;
        let name = veekun::to_pascal_case(vcsv::get_field(&record, 1)?);
        let category = vcsv::from_field(&record, 2)?;
        let cost = vcsv::from_field(&record, 3)?;
        let fling_power: VeekunOption<_> = vcsv::from_field(&record, 4)?;
        let fling_effect
            = vcsv::from_option_field(&record, 5, FlingEffect::None)?;
        self.0.insert(id, Item {
            id,
            name,
            category,
            cost,
            fling_power: fling_power.into(),
            fling_effect,
            flags: flags::Flags::empty(),
            berry: None,
        });
        Ok(())
    }
}
