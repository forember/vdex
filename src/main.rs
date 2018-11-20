extern crate enum_repr;

mod abilities;
mod items;
mod types;

use enum_repr::EnumRepr;
use abilities::Ability;
use items::Flags;
use items::FlingEffect;
use items::Pockets;
use types::Type;

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

fn main() {
    println!("{} abilities", Ability::Teravolt.repr());
    println!("{} item flags", Flags::Underground.repr());
    println!("{} item fling effects", FlingEffect::Flinch.repr());
    println!("{} item pockets", Pockets::Key.repr());
    println!("{} types", Type::Dark.repr());
    println!("{} contest types", ContestType::Tough.repr());
    println!("{} egg groups", EggGroup::NoEggs.repr());
    println!("{} evolution triggers", EvolutionTrigger::Shed.repr());
    println!("{} genders", Gender::Genderless.repr());
}
