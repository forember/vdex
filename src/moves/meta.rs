use enums::*;

#[EnumRepr(type = "i8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Ailment {
    Unknown = -1,
    None,
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison,
    Confusion,
    Infatuation,
    Trap,
    Nightmare,
    Torment = 12,
    Disable,
    Yawn,
    HealBlock,
    NoTypeImmunity = 17,
    LeechSeed,
    Embargo,
    PerishSong,
    Ingrain,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    Damage = 0,
    Ailment,
    NetGoodStats,
    Heal,
    DamageAilment,
    Swagger,
    DamageLower,
    DamageRaise,
    DamageHeal,
    OneHitKO,
    WholeFieldEffect,
    FieldEffect,
    ForceSwitch,
    Unique,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Flag {
    Contact = 1,
    Charge,
    Recharge,
    Protect,
    Reflectable,
    Snatch,
    Mirror,
    Punch,
    Sound,
    Gravity,
    Defrost,
    Distance,
    Heal,
    Authentic,
}

pub fn assert_sanity() {
    assert_eq!(Ailment::Nightmare.repr(), 9);
    assert_eq!(Ailment::HealBlock.repr(), 15);
    assert_eq!(Ailment::Ingrain.repr(), 21);
    assert_eq!(Category::Unique.repr(), 13);
    assert_eq!(Flag::Authentic.repr(), 14);
}
