#![allow(dead_code)]

extern crate enum_repr;

mod abilities;
mod items;
mod misc;
mod moves;
mod natures;
mod types;
mod versions;

pub fn assert_sanity() {
    abilities::assert_sanity();
    items::assert_sanity();
    misc::assert_sanity();
    moves::assert_sanity();
    natures::assert_sanity();
    types::assert_sanity();
    versions::assert_sanity();
}

fn main() {
    assert_sanity();
    let efficacy_path = std::path::Path::new("veekun/type_efficacy.csv");
    let _efficacy_table = types::EfficacyTable::from_csv_file(efficacy_path)
        .expect("Failed to load efficacy table CSV!");
}
