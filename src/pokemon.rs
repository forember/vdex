//! TODO: Under construction.

use std::collections::HashMap;
use Ability;
use enums::*;
use FromVeekun;
use moves::LearnMethod;
use Type;
use vcsv;
use vcsv::FromCsv;
use vdata;
use VeekunOption;
use veekun::repr::VeekunString;
use versions::{Generation, VersionGroup};

/// The number of stats that exist out of battle (all but accuracy and evasion).
pub const PERMANENT_STATS: usize = 6;
/// The total number of Pokémon species in pbirch.
pub const SPECIES_COUNT: usize = 649;
const POKEMON_COUNT: usize = 673;

/// The groups of Pokémon which can interbreed.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Egg_Group)
/// > Egg Groups (Japanese: タマゴグループ Egg Group) are categories which
/// > determine which Pokémon are able to interbreed. The concept was introduced
/// > in Generation II, along with breeding. Similar to types, a Pokémon may
/// > belong to either one or two Egg Groups.
#[EnumRepr(type = "u8")]
pub enum EggGroup {
    Monster = 1,
    Water1,
    Bug,
    Flying,
    Ground,
    Fairy,
    Plant,
    Humanshape,
    Water3,
    Mineral,
    Indeterminate,
    Water2,
    Ditto,
    Dragon,
    NoEggs,
}

/// The method by which a Pokémon evolves.
#[EnumRepr(type = "u8")]
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
}

/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Gender) The
/// > gender (Japanese: 性別 sex) of a Pokémon is a concept introduced in
/// > Generation II, though touched upon in Generation I. In Gold and Silver
/// > Versions, most species of Pokémon were assigned a gender, male (Japanese:
/// > オス male) or female (Japanese: メス female); however, the genders of some
/// > species of Pokémon were left unknown (Japanese: 不明 unknown). This
/// > feature allowed for Pokémon breeding, as well as introducing the concept
/// > of a Pokémon Egg to the series. Gender makes no difference in the stats of
/// > a Pokémon after Generation II, unless the two Pokémon are a different
/// > species entirely, such as Nidoran.
#[EnumRepr(type = "u8")]
pub enum Gender {
    Female = 1,
    Male,
    Genderless,
}

/// Either one or two elements.
pub enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

struct AbilityTable([[Option<Ability>; 3]; POKEMON_COUNT]);

impl vcsv::FromCsvIncremental for AbilityTable {
    fn from_empty_csv() -> Self {
        AbilityTable([[None; 3]; POKEMON_COUNT])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: usize = vcsv::from_field(&record, 0)?;
        let ability = vcsv::from_field(&record, 1)?;
        let slot: usize = vcsv::from_field(&record, 3)?;
        self.0[pokemon_id][slot] = Some(ability);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Form {
    pub id: u16,
    pub name: Option<String>,
    pub battle_only: bool,
}

struct FormTable(Vec<Vec<Form>>);

impl vcsv::FromCsvIncremental for FormTable {
    fn from_empty_csv() -> Self {
        FormTable(std::iter::repeat(vec![]).take(POKEMON_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        let name: VeekunOption<VeekunString> = vcsv::from_field(&record, 1)?;
        let pokemon_id: usize = vcsv::from_field(&record, 2)?;
        let battle_only = vcsv::from_field(&record, 5)?;
        self.0[pokemon_id].push(Form {
            id,
            name: name.into(),
            battle_only,
        });
        Ok(())
    }
}

/// A Pokémon's base permanent stats.
pub struct BaseStats([u8; PERMANENT_STATS]);

struct StatsTable([BaseStats; POKEMON_COUNT]);

pub struct Pokemon {
    pub id: u16,
    pub abilities: OneOrTwo<Ability>,
    pub hidden_ability: Option<Ability>,
    pub forms: Vec<Form>,
    pub stats: [u8; PERMANENT_STATS],
}

struct PokemonTable([Vec<Pokemon>; SPECIES_COUNT]);

struct EggGroupTable([Vec<EggGroup>; SPECIES_COUNT]);

pub struct EvolvesFrom {
    pub from_id: u16,
    pub trigger: EvolutionTrigger,
    pub level: Option<u8>,
    pub gender: Gender,
    pub move_id: u16,
    pub relative_physical_stats: i8,
}

struct EvolutionTable(HashMap<u16, EvolvesFrom>);

pub struct PokemonMove {
    pub move_id: u16,
    pub learn_method: LearnMethod,
    pub level: u8,
}

struct PokemonMoveTable([HashMap<VersionGroup, Vec<PokemonMove>>; SPECIES_COUNT]);

struct TypeTable([Vec<Type>; POKEMON_COUNT]);

pub struct Species {
    pub name: String,
    pub generation: Generation,
    pub pokemon: Vec<Pokemon>,
    pub egg_groups: OneOrTwo<EggGroup>,
    pub evolves_from: Option<EvolvesFrom>,
    pub moves: HashMap<VersionGroup, Vec<PokemonMove>>,
    pub types: OneOrTwo<Type>,
}

pub struct SpeciesTable(pub [Species; SPECIES_COUNT]);
