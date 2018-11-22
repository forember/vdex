extern crate enum_repr;
extern crate rand;

pub mod abilities;
pub mod enums;
pub mod items;
pub mod moves;
pub mod natures;
pub mod pokemon;
pub mod types;
pub mod veekun;
pub mod versions;

#[cfg(test)]
mod tests {
    use abilities;
    use items;
    use moves;
    use natures;
    use pokemon;
    use types;
    use versions;

    use std::path::Path;
    use veekun::csv::FromCsv;

    #[test]
    fn assert_sanity() {
        abilities::assert_sanity();
        items::assert_sanity();
        moves::assert_sanity();
        natures::assert_sanity();
        pokemon::assert_sanity();
        types::assert_sanity();
        versions::assert_sanity();
    }

    fn load_palace() -> natures::PalaceTable {
        let path = Path::new("veekun/nature_battle_style_preferences.csv");
        natures::PalaceTable::from_csv_file(path)
            .expect("Failed to load palace table CSV!")
    }

    fn load_efficacy() -> types::EfficacyTable {
        let path = Path::new("veekun/type_efficacy.csv");
        types::EfficacyTable::from_csv_file(path)
            .expect("Failed to load efficacy table CSV!")
    }
    
    #[test]
    fn load_all() {
        load_palace();
        load_efficacy();
    }
    
    #[test]
    #[cfg(palace)]
    fn print_palace() {
        let table = load_palace();
        for nature_id in 0..25 {
            let nature = natures::Nature::from_repr(nature_id).unwrap();
            let high_attack = table.high.attack[nature_id];
            let high_defense = table.high.defense[nature_id];
            let low_attack = table.low.attack[nature_id];
            let low_defense = table.low.defense[nature_id];
            println!("{:?}: high({:?}%, {:?}%), low({:?}%, {:?}%)",
                nature, high_attack, high_defense, low_attack, low_defense);
        }
    }

    #[test]
    #[cfg(efficacy)]
    fn print_efficacy() {
        let table = load_efficacy();
        for damage_id in 0..17 {
            let damage = types::Type::from_repr(damage_id).unwrap();
            for target_id in 0..17 {
                let target = types::Type::from_repr(target_id).unwrap();
                let efficacy = table.efficacy(damage, target);
                if efficacy == types::Efficacy::Regular {
                    continue;
                }
                println!("{:?} is {:?} effective against {:?}.",
                    damage, efficacy, target);
            }
        }
    }
}
