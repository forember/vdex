//! TODO: Under construction.

use std::collections::HashMap;
use Ability;
use enums::*;
use moves::LearnMethod;
use Type;
use versions::{Generation, VersionGroup};

pub const PERMANENT_STATS: usize = 6;
pub const SPECIES_COUNT: usize = 649;
const POKEMON_COUNT: usize = 673;

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

#[EnumRepr(type = "u8")]
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
}

#[EnumRepr(type = "u8")]
pub enum Gender {
    Female = 1,
    Male,
    Genderless,
}

pub enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

struct AbilityTable([[Option<Ability>; 3]; POKEMON_COUNT]);

pub struct Form {
    pub id: u16,
    pub name: Option<String>,
    pub battle_only: bool,
}

struct FormTable([Vec<Form>; POKEMON_COUNT]);

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
    pub egg_groups: OneOrTwo<EggGroup>,
    pub evolves_from: Option<EvolvesFrom>,
    pub moves: HashMap<VersionGroup, Vec<PokemonMove>>,
    pub types: OneOrTwo<Type>,
}

pub struct SpeciesTable(pub [Species; SPECIES_COUNT]);
