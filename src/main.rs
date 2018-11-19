extern crate enum_repr;
extern crate libc;

use libc::*;

use enum_repr::EnumRepr;


#[EnumRepr(type = "c_int")]
#[derive(Debug, PartialEq)]
pub enum Test {
    A,
    B,
    C = 5,
    D,
}

fn main() {
    assert_eq!(Test::B.repr(), 1);
    assert_eq!(Test::from_repr(6), Some(Test::D));
    assert!(Test::from_repr(2).is_none());
}

/*
#[EnumRepr(type = "u8")]
#[derive(Debug)]
pub enum Abilities {
    Stench = 1,
    Drizzle,
    SpeedBoost,
}

fn main() {
    let ability = Abilities::from_repr(3);
    println!("ability: {:?}", ability);
}
*/
