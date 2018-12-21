use enums::*;

#[EnumRepr(type = "i8")]
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

#[EnumRepr(type = "u8")]
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

bitflags! {
    pub struct Flags: u16 {
        const Contact = 0x0001;
        const Charge = 0x0002;
        const Recharge = 0x0004;
        const Protect = 0x0008;
        const Reflectable = 0x0010;
        const Snatch = 0x0020;
        const Mirror = 0x0040;
        const Punch = 0x0080;
        const Sound = 0x0100;
        const Gravity = 0x0200;
        const Defrost = 0x0400;
        const Distance = 0x0800;
        const Heal = 0x1000;
        const Authentic = 0x2000;
    }
}

pub fn assert_sanity() {
    assert_eq!(Ailment::Nightmare.repr(), 9);
    assert_eq!(Ailment::HealBlock.repr(), 15);
    assert_eq!(Ailment::Ingrain.repr(), 21);
    assert_eq!(Category::Unique.repr(), 13);
}
