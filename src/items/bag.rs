use crate::enums::*;
use crate::FromVeekun;

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

impl Default for Category {
    fn default() -> Self { Category::Unused }
}

impl FromVeekun for Category {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Category::from_repr(value)
    }
}

/// Bag pocket in which items are stored.
#[EnumRepr(type = "u8")]
pub enum Pocket {
    Misc = 0,
    Medicine,
    Pokeballs,
    Machines,
    Berries,
    Mail,
    Battle,
    Key,
}

impl Default for Pocket {
    fn default() -> Self { Pocket::Misc }
}

impl FromVeekun for Pocket {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}
