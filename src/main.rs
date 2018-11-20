extern crate enum_repr;

mod abilities;
mod items;
mod types;

use enum_repr::EnumRepr;
use abilities::Ability;
use items::Pockets;
use types::Type;

#[EnumRepr(type = "u8", implicit = true)]
#[derive(Debug)]
pub enum ContestType {
    Cool = 1,
    Beauty,
    Cute,
    Smart,
    Tough,
}

fn main() {
    println!("{:?} abilities", Ability::Teravolt.repr());
    println!("{:?} item pockets", Pockets::Key.repr());
    println!("{:?} types", Type::Dark.repr());
    println!("{:?} contest types", ContestType::Tough.repr()); 
}
