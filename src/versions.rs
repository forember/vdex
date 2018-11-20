use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum Version {
    Red = 1,
    Blue,
    Yellow,
    Gold,
    Silver,
    Crystal,
    Ruby,
    Sapphire,
    Emerald,
    Firered,
    Leafgreen,
    Diamond,
    Pearl,
    Platinum,
    Heartgold,
    Soulsilve,
    Black,
    White,
    Colosseum,
    XD,
    Black2,
    White2,
}
