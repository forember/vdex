use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum Type {
    Normal = 1,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
}
