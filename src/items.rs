use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum Flags {
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
pub enum Pockets {
    Misc = 1,
    Medicine,
    Pokeballs,
    Machines,
    Berries,
    Mail,
    Battle,
    Key,
}
