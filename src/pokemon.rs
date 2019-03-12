//! TODO: Under construction.

use std::collections::HashMap;
use std::iter::repeat;
use Ability;
use enums::*;
use FromVeekun;
use moves::LearnMethod;
use Stat;
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
#[derive(Clone, Debug)]
pub enum OneOrTwo<T: Clone> {
    One(T),
    Two(T, T),
}

impl FromVeekun for EggGroup {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
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
        FormTable(repeat(vec![]).take(POKEMON_COUNT).collect::<Vec<_>>())
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
#[derive(Copy, Clone, Debug)]
pub struct BaseStats(pub [u8; PERMANENT_STATS]);

impl Default for BaseStats {
    fn default() -> Self {
        BaseStats([0; PERMANENT_STATS])
    }
}

impl std::ops::Index<Stat> for BaseStats {
    type Output = u8;

    fn index<'a>(&'a self, index: Stat) -> &'a u8 {
        &self.0[(index.repr() + 1) as usize]
    }
}

impl std::ops::IndexMut<Stat> for BaseStats {
    fn index_mut<'a>(&'a mut self, index: Stat) -> &'a mut u8 {
        &mut self.0[(index.repr() + 1) as usize]
    }
}

struct StatsTable([BaseStats; POKEMON_COUNT]);

impl vcsv::FromCsvIncremental for StatsTable {
    fn from_empty_csv() -> Self {
        StatsTable([Default::default(); POKEMON_COUNT])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let stat = vcsv::from_field(&record, 1)?;
        let base = vcsv::from_field(&record, 2)?;
        self.0[id][stat] = base;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub id: u16,
    pub abilities: OneOrTwo<Ability>,
    pub hidden_ability: Option<Ability>,
    pub forms: Vec<Form>,
    pub stats: BaseStats,
}

struct PokemonTable(Vec<Vec<Pokemon>>);

impl vcsv::FromCsvIncremental for PokemonTable {
    fn from_empty_csv() -> Self {
        PokemonTable(repeat(vec![]).take(SPECIES_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: u16 = vcsv::from_field(&record, 0)?;
        let species_id: usize = vcsv::from_field(&record, 1)?;
        self.0[species_id].push(Pokemon {
            id: pokemon_id,
            abilities: OneOrTwo::One(Ability::Stench),
            hidden_ability: None,
            forms: vec![],
            stats: Default::default(),
        });
        Ok(())
    }
}

impl PokemonTable {
    fn new() -> Self {
        let ability_table
            = AbilityTable::from_csv_data(vdata::ABILITIES).unwrap();
        let form_table = FormTable::from_csv_data(vdata::FORMS).unwrap();
        let stats_table = StatsTable::from_csv_data(vdata::STATS).unwrap();
        let mut pokemon_table
            = PokemonTable::from_csv_data(vdata::POKEMON).unwrap();
        pokemon_table.set_abilities(&ability_table);
        pokemon_table.set_forms(&form_table);
        pokemon_table.set_stats(&stats_table);
        pokemon_table
    }

    fn set_abilities(&mut self, ability_table: &AbilityTable) {
        for mut species in self.0.iter_mut() {
            for mut pokemon in species {
                let id = pokemon.id as usize;
                let first = ability_table.0[id][0].unwrap();
                pokemon.abilities = match ability_table.0[id][1] {
                    Some(second) => OneOrTwo::Two(first, second),
                    None => OneOrTwo::One(first),
                };
                pokemon.hidden_ability = ability_table.0[id][2];
            }
        }
    }

    fn set_forms(&mut self, form_table: &FormTable) {
        for mut species in self.0.iter_mut() {
            for mut pokemon in species {
                let id = pokemon.id as usize;
                pokemon.forms = form_table.0[id].clone();
            }
        }
    }

    fn set_stats(&mut self, stats_table: &StatsTable) {
        for mut species in self.0.iter_mut() {
            for mut pokemon in species {
                let id = pokemon.id as usize;
                pokemon.stats = stats_table.0[id];
            }
        }
    }
}

struct EggGroupTable(Vec<Vec<EggGroup>>);

impl vcsv::FromCsvIncremental for EggGroupTable {
    fn from_empty_csv() -> Self {
        EggGroupTable(repeat(vec![]).take(SPECIES_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let egg_group = vcsv::from_field(&record, 1)?;
        self.0[id].push(egg_group);
        Ok(())
    }
}

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

struct PokemonMoveTable(Vec<HashMap<VersionGroup, Vec<PokemonMove>>>);

struct TypeTable(Vec<Vec<Type>>);

pub struct Species {
    pub name: String,
    pub generation: Generation,
    pub pokemon: Vec<Pokemon>,
    pub egg_groups: OneOrTwo<EggGroup>,
    pub evolves_from: Option<EvolvesFrom>,
    pub moves: HashMap<VersionGroup, Vec<PokemonMove>>,
    pub types: OneOrTwo<Type>,
}

pub struct SpeciesTable(Vec<Species>);
