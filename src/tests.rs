use Ability;
use Efficacy;
use Nature;
use Type;
use Stat;
use items;
use moves;
use natures;
use pokemon;
use types;
use versions;

use Enum;
use vcsv::FromCsv;
use to_pascal_case;

use std::mem::size_of;
use std::path::Path;

#[test]
fn assert_sanity() {
    assert_eq!(Ability::Teravolt.repr(), 164);
    assert_eq!(Efficacy::Super.repr(), 1);
    assert_eq!(Nature::Quirky.repr(), 24);
    assert_eq!(Type::Dark.repr(), 16);
    assert_eq!(Stat::Evasion.repr(), 6);
    assert_eq!(items::Category::StatusCures.repr(), 30);
    assert_eq!(items::Category::MiracleShooter.repr(), 43);
    assert_eq!(items::Flavor::Bitter.repr(), 4);
    assert_eq!(items::FlingEffect::Flinch.repr(), 7);
    assert_eq!(items::Pocket::Key.repr(), 8);
    assert_eq!(moves::Ailment::Nightmare.repr(), 9);
    assert_eq!(moves::Ailment::HealBlock.repr(), 15);
    assert_eq!(moves::Ailment::Ingrain.repr(), 21);
    assert_eq!(moves::BattleStyle::Support.repr(), 3);
    assert_eq!(moves::Category::Unique.repr(), 13);
    assert_eq!(moves::DamageClass::Special.repr(), 3);
    assert_eq!(moves::Effect::RaiseUserDefense.repr(), 12);
    assert_eq!(moves::Effect::LowerTargetSpeed.repr(), 21);
    assert_eq!(moves::Effect::RaiseUserSpecialDefense2.repr(), 55);
    assert_eq!(moves::Effect::LowerTargetSpeed2.repr(), 61);
    assert_eq!(moves::Effect::ChanceLowerTargetAccuracy.repr(), 74);
    assert_eq!(moves::Effect::Sketch.repr(), 96);
    assert_eq!(moves::Effect::Curse.repr(), 110);
    assert_eq!(moves::Effect::Sonicboom.repr(), 131);
    assert_eq!(moves::Effect::ChanceRaiseUserAllStats.repr(), 141);
    assert_eq!(moves::Effect::DefenseCurl.repr(), 157);
    assert_eq!(moves::Effect::Swallow.repr(), 163);
    assert_eq!(moves::Effect::Bounce.repr(), 264);
    assert_eq!(moves::Effect::IceBurn.repr(), 333);
    assert_eq!(moves::Effect::Hurricane.repr(), 338);
    assert_eq!(moves::LearnMethod::FormChange.repr(), 10);
    assert_eq!(moves::Target::EntireField.repr(), 12);
    assert_eq!(pokemon::EggGroup::NoEggs.repr(), 15);
    assert_eq!(pokemon::EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(pokemon::Gender::Genderless.repr(), 3);
    assert_eq!(versions::Generation::V.repr(), 5);
    assert_eq!(versions::Version::White2.repr(), 22);
    assert_eq!(versions::VersionGroup::BlackWhite2.repr(), 14);
}

#[test]
fn check_pascal_case() {
    assert_eq!(to_pascal_case("master-ball"), "MasterBall");
}

fn load_items() -> items::ItemTable {
    items::ItemTable::from_files(
        "veekun/data/items.csv",
        "veekun/data/item_flag_map.csv",
        "veekun/data/berries.csv",
        "veekun/data/berry_flavors.csv")
        .expect("Failed to load item CSV files!")
}

fn load_moves() -> moves::MoveTable {
    moves::MoveTable::from_files(
        "veekun/data/moves.csv",
        "veekun/data/move_meta.csv",
        "veekun/data/move_meta_stat_changes.csv",
        "veekun/data/move_flag_map.csv")
        .expect("Failed to load move CSV files!")
}

fn load_palace() -> natures::PalaceTable {
    let path = Path::new("veekun/data/nature_battle_style_preferences.csv");
    natures::PalaceTable::from_csv_file(path)
        .expect("Failed to load palace table CSV!")
}

fn load_efficacy() -> types::EfficacyTable {
    let path = Path::new("veekun/data/type_efficacy.csv");
    types::EfficacyTable::from_csv_file(path)
        .expect("Failed to load efficacy table CSV!")
}

#[test]
fn load_all() {
    load_items();
    load_moves();
    load_palace();
    load_efficacy();
}

#[test]
#[ignore]
fn print_items() {
    let table = load_items();
    let mut v: Vec<_> = table.0.iter().collect();
    v.as_mut_slice().sort_unstable_by_key(|p| p.0);
    for (_id, item) in v.into_iter() {
        eprintln!("{:?}", item);
    }
    panic!("Output from this test must be manually inspected.");
}

#[test]
#[ignore]
fn print_moves() {
    let table = load_moves();
    for mov in &table.0 {
        eprintln!("{:?}", mov);
    }
    panic!("Output from this test must be manually inspected.");
}

#[test]
#[ignore]
fn print_palace() {
    let table = load_palace();
    for nature_id in 0..25 {
        let nature = natures::Nature::from_repr(nature_id).unwrap();
        let i = nature_id as usize;
        let high_attack = table.high.attack[i];
        let high_defense = table.high.defense[i];
        let low_attack = table.low.attack[i];
        let low_defense = table.low.defense[i];
        eprintln!("{:?}: high({:?}%, {:?}%), low({:?}%, {:?}%)",
            nature, high_attack, high_defense, low_attack, low_defense);
    }
    panic!("Output from this test must be manually inspected.");
}

#[test]
#[ignore]
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
            eprintln!("{:?} is {:?} effective against {:?}.",
                damage, efficacy, target);
        }
    }
    panic!("Output from this test must be manually inspected.");
}

#[test]
#[ignore]
fn sizes() {
    assert_eq!(size_of::<items::Berry>(), 6);
    assert_eq!(size_of::<items::Item>(), 40);
    assert_eq!(size_of::<moves::Meta>(), 24);
    assert_eq!(size_of::<moves::Move>(), 64);
}
