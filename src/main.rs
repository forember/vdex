extern crate enum_repr;

mod abilities;
mod items;
mod misc;
mod moves;
mod types;
mod versions;

pub fn assert_sanity() {
    abilities::assert_sanity();
    items::assert_sanity();
    misc::assert_sanity();
    moves::assert_sanity();
    types::assert_sanity();
    versions::assert_sanity();
}

fn main() {
    assert_sanity();
    let path = std::path::Path::new("veekun/type_efficacy.csv");
    let table = types::EfficacyTable::from_csv_file(path)
        .expect("Failed to load efficacy table CSV!");
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
