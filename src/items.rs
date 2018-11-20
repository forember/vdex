use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug)]
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
#[derive(Debug)]
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
