pub mod effects;
pub mod meta;

use enums::*;
use FromVeekun;
use natures::Stat;
use types::Type;
use versions::Generation;

#[EnumRepr(type = "u8")]
pub enum BattleStyle {
    Attack = 1,
    Defense,
    Support,
}

#[EnumRepr(type = "u8")]
pub enum DamageClass {
    NonDamaging = 1,
    Physical,
    Special,
}

#[EnumRepr(type = "u8")]
pub enum LearnMethod {
    LevelUp = 1,
    Egg,
    Tutor,
    Machine,
    StadiumSurfingPikachu,
    LightBallEgg,
    ColosseumPurification,
    XDShadow,
    XDPurification,
    FormChange,
}

#[EnumRepr(type = "u8")]
pub enum Target {
    SpecificMove = 1,
    SelectedPokemonReuseStolen,
    Ally,
    UsersField,
    UserOrAlly,
    OpponentsField,
    User,
    RandomOpponent,
    AllOtherPokemon,
    SelectedPokemon,
    AllOpponents,
    EntireField,
}

pub fn assert_sanity() {
    assert_eq!(BattleStyle::Support.repr(), 3);
    assert_eq!(DamageClass::Special.repr(), 3);
    assert_eq!(LearnMethod::FormChange.repr(), 10);
    assert_eq!(Target::EntireField.repr(), 12);
    effects::assert_sanity();
    meta::assert_sanity();
}

impl FromVeekun for BattleStyle {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for DamageClass {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for LearnMethod {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for Target {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

pub struct Move {
    pub id: u16,
    pub name: String,
    pub generation: Generation,
    pub typ: Type,
    pub power: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub target: Target,
    pub damage_class: DamageClass,
    pub effect: effects::MoveEffect,
    pub effect_chance: Option<u8>,
    pub flags: meta::Flags,
    pub category: meta::Category,
    pub ailment: meta::Ailment,
    pub hits: Option<(u8, u8)>,
    pub turns: Option<(u8, u8)>,
    pub recoil: i8,
    pub healing: i8,
    pub critical_rate: i8,
    pub ailment_chance: u8,
    pub flinch_chance: u8,
    pub stat_chance: u8,
    pub stat_changes: Vec<(Stat, i8)>,
}
