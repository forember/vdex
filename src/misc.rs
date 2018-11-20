use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum ContestType {
    Cool = 1,
    Beauty,
    Cute,
    Smart,
    Tough,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum EggGroup {
    Monster = 1,
    Water1,
    Bug,
    Flying,
    Ground,
    Fairy,
    Plant,
    Humanshape,
    Water3,
    Mineral,
    Indeterminate,
    Water2,
    Ditto,
    Dragon,
    NoEggs,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug, PartialEq)]
pub enum Gender {
    Female = 1,
    Male,
    Genderless,
}

pub fn assert_sanity() {
    assert_eq!(ContestType::Tough.repr(), 5);
    assert_eq!(EggGroup::NoEggs.repr(), 15);
    assert_eq!(EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(Gender::Genderless.repr(), 3);
}
