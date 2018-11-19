extern crate enum_repr;

use enum_repr::EnumRepr;

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
