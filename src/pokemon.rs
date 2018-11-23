use enums::*;

enum_repr!("u8";
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
});

enum_repr!("u8";
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
});

enum_repr!("u8";
pub enum Gender {
    Female = 1,
    Male,
    Genderless,
});

pub fn assert_sanity() {
    assert_eq!(EggGroup::NoEggs.repr(), 15);
    assert_eq!(EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(Gender::Genderless.repr(), 3);
}
