extern crate enum_repr;

mod abilities;
mod items;
mod moves;
mod move_effects;
mod move_meta;
mod types;
mod versions;

use enum_repr::EnumRepr;
use abilities::Ability;
use types::Type;
use versions::Version;

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
    assert_eq!(Ability::Teravolt.repr(), 164);
    assert_eq!(items::Flags::Underground.repr(), 8);
    assert_eq!(items::FlingEffect::Flinch.repr(), 7);
    assert_eq!(items::Pockets::Key.repr(), 8);
    assert_eq!(moves::BattleStyle::Support.repr(), 3);
    assert_eq!(moves::DamageClass::Special.repr(), 3);
    assert_eq!(moves::LearnMethod::FormChange.repr(), 10);
    assert_eq!(moves::Target::EntireField.repr(), 12);
    move_effects::assert_sanity();
    move_meta::assert_sanity();
    assert_eq!(Type::Dark.repr(), 17);
    assert_eq!(Version::White2.repr(), 22);
    assert_eq!(ContestType::Tough.repr(), 5);
    assert_eq!(EggGroup::NoEggs.repr(), 15);
    assert_eq!(EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(Gender::Genderless.repr(), 3);
}

fn main() {
    assert_sanity();
}
