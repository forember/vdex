use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum BattleStyle {
    Attack = 1,
    Defense,
    Support,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum DamageClass {
    NonDamaging = 1,
    Physical,
    Special,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
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

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
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
