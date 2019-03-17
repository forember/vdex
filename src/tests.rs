use crate::Ability;
use crate::Efficacy;
use crate::Nature;
use crate::Type;
use crate::Stat;
use crate::items;
use crate::moves;
use crate::pokemon;
use crate::versions;

use crate::Enum;
use crate::pokedex;
use crate::to_pascal_case;

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
    assert_eq!(items::Pocket::Key.repr(), 7);
    assert_eq!(moves::Ailment::Nightmare.repr(), 9);
    assert_eq!(moves::Ailment::HealBlock.repr(), 15);
    assert_eq!(moves::Ailment::Ingrain.repr(), 21);
    assert_eq!(moves::BattleStyle::Support.repr(), 2);
    assert_eq!(moves::Category::Unique.repr(), 13);
    assert_eq!(moves::DamageClass::Special.repr(), 2);
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
    assert_eq!(moves::LearnMethod::FormChange.repr(), 9);
    assert_eq!(moves::Target::EntireField.repr(), 11);
    assert_eq!(pokemon::EggGroup::NoEggs.repr(), 15);
    assert_eq!(pokemon::EvolutionTrigger::Shed.repr(), 4);
    assert_eq!(pokemon::Gender::Genderless.repr(), 3);
    assert_eq!(versions::Generation::V.repr(), 4);
    assert_eq!(versions::Version::White2.repr(), 21);
    assert_eq!(versions::VersionGroup::BlackWhite2.repr(), 13);
}

#[test]
fn check_pascal_case() {
    assert_eq!(to_pascal_case("master-ball"), "MasterBall");
}

#[test]
fn load_pokedex() {
    pokedex();
}
