use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
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

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum Flag {
    Countable = 1,
    Consumable,
    UsableOverworld,
    UsableInBattle,
    Holdable,
    HoldablePassive,
    HoldableActive,
    Underground,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum FlingEffect {
    BadlyPoison = 1,
    Burn,
    ActivateBerry,
    ActivateHerb,
    Paralyze,
    Poison,
    Flinch,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
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
    assert_eq!(Flag::Underground.repr(), 8);
    assert_eq!(FlingEffect::Flinch.repr(), 7);
    assert_eq!(Pocket::Key.repr(), 8);
}

impl Category {
    pub fn pocket(&self) -> Pocket {
        let x: u8 = self.repr();
        match x {
            9 ... 19 | 24 | 32 | 35 | 36 | 42 => Pocket::Misc,
            26 ... 30 => Pocket::Medicine,
            33 | 34 | 39 => Pocket::Pokeballs,
            37 => Pocket::Machines,
            2 ... 8 => Pocket::Berries,
            25 => Pocket::Mail,
            1 | 38 | 43 => Pocket::Battle,
            20 ... 23 | 40 | 41 => Pocket::Key,
            _ => panic!("Invalid return from Category::repr!"),
        }
    }
}
