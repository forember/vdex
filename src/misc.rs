use enum_repr::EnumRepr;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContestType {
    Cool = 1,
    Beauty,
    Cute,
    Smart,
    Tough,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Flavor {
    Spicy = 1,
    Dry,
    Sweet,
    Bitter,
    Sour,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Gender {
    Female = 1,
    Male,
    Genderless,
}

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Stat {
    HP = 1,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasion,
}

impl std::convert::From<Flavor> for ContestType {
    fn from(flavor: Flavor) -> Self {
        ContestType::from_repr(flavor.repr()).unwrap()
    }
}

impl std::convert::From<ContestType> for Flavor {
    fn from(contest: ContestType) -> Self {
        Flavor::from_repr(contest.repr()).unwrap()
    }
}

pub fn assert_sanity() {
    assert_eq!(ContestType::Tough.repr(), 5);
    assert_eq!(EggGroup::NoEggs.repr(), 15);
    assert_eq!(EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(Flavor::Sour.repr(), 5);
    assert_eq!(Gender::Genderless.repr(), 3);
    assert_eq!(Stat::Evasion.repr(), 8);
}
