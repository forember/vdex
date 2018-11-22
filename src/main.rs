#![allow(dead_code)]

extern crate enum_repr;
extern crate rand;

mod abilities;
mod items;
mod misc;
mod moves;
mod natures;
mod types;
mod veekun;
mod versions;

use std::path::Path;
use veekun::csv::FromCsv;

pub fn assert_sanity() {
    abilities::assert_sanity();
    items::assert_sanity();
    misc::assert_sanity();
    moves::assert_sanity();
    natures::assert_sanity();
    types::assert_sanity();
    versions::assert_sanity();
}

fn _eprint_efficacy_table(table: &types::EfficacyTable) {
    for damage_id in 0..17 {
        for target_id in 0..17 {
            let damage = types::Type::from_repr(damage_id).unwrap();
            let target = types::Type::from_repr(target_id).unwrap();
            let efficacy = table.efficacy(damage, target);
            if efficacy == types::Efficacy::Regular {
                continue;
            }
            eprintln!("{:?} is {:?} effective against {:?}.",
                damage, efficacy, target);
        }
    }
}

fn main() {
    assert_sanity();
    let efficacy_path = Path::new("veekun/type_efficacy.csv");
    let _efficacy_table = types::EfficacyTable::from_csv_file(efficacy_path)
        .expect("Failed to load efficacy table CSV!");
    //_eprint_efficacy_table(&_efficacy_table);
    let palace_path = Path::new("veekun/nature_battle_style_preferences.csv");
    let _palace_path = natures::PalaceTable::from_csv_file(palace_path)
        .expect("Failed to load palace table CSV!");
}
