extern crate enum_repr;

mod abilities;
mod items;
mod misc;
mod moves;
mod types;
mod versions;

use abilities::Ability;
use types::Type;


pub fn assert_sanity() {
    items::assert_sanity();
    misc::assert_sanity();
    moves::assert_sanity();
    versions::assert_sanity();
    assert_eq!(Ability::Teravolt.repr(), 164);
    assert_eq!(Type::Dark.repr(), 17);
}

fn main() {
    assert_sanity();
}
