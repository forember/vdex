pub mod effects;
pub mod meta;

use enums::*;
use veekun;

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

impl veekun::FromVeekun<u8> for BattleStyle {
    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl veekun::FromVeekun<u8> for DamageClass {
    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl veekun::FromVeekun<u8> for LearnMethod {
    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl veekun::FromVeekun<u8> for Target {
    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}
