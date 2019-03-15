//! TODO: Under construction.

use std::collections::HashMap;
use std::iter::repeat;
use crate::Ability;
use crate::enums::*;
use crate::FromVeekun;
use crate::moves::LearnMethod;
use crate::Stat;
use crate::to_pascal_case;
use crate::Type;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use crate::VeekunOption;
use veekun::repr::VeekunString;
use crate::versions::{Generation, VersionGroup};

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
#[derive(Copy, Clone, Debug)]
pub enum OneOrTwo<T: Copy> {
    One(T),
    Two(T, T),
}

impl<T: Copy> OneOrTwo<T> {
    fn from_options(array: [Option<T>; 2]) -> Option<Self> {
        array[0].and_then(|first| Some(match array[1] {
            Some(second) => OneOrTwo::Two(first, second),
            None => OneOrTwo::One(first),
        })).or_else(|| array[1].and_then(|second| Some(OneOrTwo::One(second))))
    }
}

impl FromVeekun for EggGroup {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

impl FromVeekun for EvolutionTrigger {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

impl FromVeekun for Gender {
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
        self.0[pokemon_id - 1][slot - 1] = Some(ability);
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
        FormTable(repeat(Vec::new()).take(POKEMON_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        let name: VeekunOption<VeekunString> = vcsv::from_field(&record, 1)?;
        let pokemon_id: usize = vcsv::from_field(&record, 2)?;
        let battle_only: u8 = vcsv::from_field(&record, 5)?;
        self.0[pokemon_id - 1].push(Form {
            id,
            name: name.into(),
            battle_only: battle_only != 0,
        });
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PokemonMove {
    pub move_id: u16,
    pub learn_method: LearnMethod,
    pub level: u8,
}

struct PokemonMoveTable(Vec<HashMap<VersionGroup, Vec<PokemonMove>>>);

impl vcsv::FromCsvIncremental for PokemonMoveTable {
    fn from_empty_csv() -> Self {
        PokemonMoveTable(repeat(HashMap::new())
                .take(POKEMON_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: usize = vcsv::from_field(&record, 0)?;
        let version_group = vcsv::from_field(&record, 1)?;
        let move_id = vcsv::from_field(&record, 2)?;
        let learn_method = vcsv::from_field(&record, 3)?;
        let level = vcsv::from_field(&record, 4)?;
        let pokemon_move = PokemonMove { move_id, learn_method, level };
        self.0[pokemon_id - 1].entry(version_group)
            .or_insert(Vec::new()).push(pokemon_move);
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

struct StatTable([BaseStats; POKEMON_COUNT]);

impl vcsv::FromCsvIncremental for StatTable {
    fn from_empty_csv() -> Self {
        StatTable([Default::default(); POKEMON_COUNT])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let stat = vcsv::from_field(&record, 1)?;
        let base = vcsv::from_field(&record, 2)?;
        self.0[id - 1][stat] = base;
        Ok(())
    }
}

struct TypeTable([[Option<Type>; 2]; POKEMON_COUNT]);

impl vcsv::FromCsvIncremental for TypeTable {
    fn from_empty_csv() -> Self {
        TypeTable([[None; 2]; POKEMON_COUNT])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: usize = vcsv::from_field(&record, 0)?;
        let typ = vcsv::from_field(&record, 1)?;
        let slot: usize = vcsv::from_field(&record, 2)?;
        self.0[pokemon_id - 1][slot - 1] = Some(typ);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub id: u16,
    pub abilities: OneOrTwo<Ability>,
    pub hidden_ability: Option<Ability>,
    pub forms: Vec<Form>,
    pub moves: HashMap<VersionGroup, Vec<PokemonMove>>,
    pub stats: BaseStats,
    pub types: OneOrTwo<Type>,
}

struct PokemonTable(Vec<Vec<Pokemon>>);

impl vcsv::FromCsvIncremental for PokemonTable {
    fn from_empty_csv() -> Self {
        PokemonTable(repeat(Vec::new()).take(SPECIES_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: u16 = vcsv::from_field(&record, 0)?;
        let species_id: usize = vcsv::from_field(&record, 1)?;
        self.0[species_id - 1].push(Pokemon {
            id: pokemon_id,
            abilities: OneOrTwo::One(Ability::Stench),
            hidden_ability: None,
            forms: Vec::new(),
            moves: HashMap::new(),
            stats: Default::default(),
            types: OneOrTwo::One(Type::Normal),
        });
        Ok(())
    }
}

impl PokemonTable {
    fn new() -> Self {
        let ability_table
            = AbilityTable::from_csv_data(vdata::ABILITIES).unwrap();
        let form_table = FormTable::from_csv_data(vdata::FORMS).unwrap();
        let move_table
            = PokemonMoveTable::from_csv_data(vdata::POKEMON_MOVES).unwrap();
        let stat_table = StatTable::from_csv_data(vdata::STATS).unwrap();
        let type_table = TypeTable::from_csv_data(vdata::TYPES).unwrap();
        let mut pokemon_table
            = PokemonTable::from_csv_data(vdata::POKEMON).unwrap();
        pokemon_table.set_abilities(&ability_table);
        pokemon_table.set_forms(&form_table);
        pokemon_table.set_moves(&move_table);
        pokemon_table.set_types(&type_table);
        pokemon_table.set_stats(&stat_table);
        pokemon_table
    }

    fn set_abilities(&mut self, ability_table: &AbilityTable) {
        for species in self.0.iter_mut() {
            for mut pokemon in species {
                let i = (pokemon.id - 1) as usize;
                let options = [ability_table.0[i][0], ability_table.0[i][0]];
                pokemon.abilities = OneOrTwo::from_options(options).unwrap();
                pokemon.hidden_ability = ability_table.0[i][2];
            }
        }
    }

    fn set_forms(&mut self, form_table: &FormTable) {
        for species in self.0.iter_mut() {
            for mut pokemon in species {
                let i = (pokemon.id - 1) as usize;
                pokemon.forms = form_table.0[i].clone();
            }
        }
    }

    fn set_moves(&mut self, move_table: &PokemonMoveTable) {
        for species in self.0.iter_mut() {
            for mut pokemon in species {
                let i = (pokemon.id - 1) as usize;
                pokemon.moves = move_table.0[i].clone();
            }
        }
    }

    fn set_types(&mut self, type_table: &TypeTable) {
        for species in self.0.iter_mut() {
            for mut pokemon in species {
                let i = (pokemon.id - 1) as usize;
                let options = type_table.0[i];
                pokemon.types = OneOrTwo::from_options(options).unwrap();
            }
        }
    }

    fn set_stats(&mut self, stat_table: &StatTable) {
        for species in self.0.iter_mut() {
            for mut pokemon in species {
                let i = (pokemon.id - 1) as usize;
                pokemon.stats = stat_table.0[i];
            }
        }
    }
}

struct EggGroupTable(Vec<Vec<EggGroup>>);

impl vcsv::FromCsvIncremental for EggGroupTable {
    fn from_empty_csv() -> Self {
        EggGroupTable(repeat(Vec::new()).take(SPECIES_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let egg_group = vcsv::from_field(&record, 1)?;
        self.0[id - 1].push(egg_group);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EvolvesFrom {
    pub from_id: u16,
    pub trigger: EvolutionTrigger,
    pub level: u8,
    pub gender: Gender,
    pub move_id: u16,
    pub relative_physical_stats: Option<i8>,
}

struct EvolutionTable(HashMap<u16, EvolvesFrom>);

impl vcsv::FromCsvIncremental for EvolutionTable {
    fn from_empty_csv() -> Self {
        EvolutionTable(HashMap::new())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let species_id: u16 = vcsv::from_field(&record, 1)?;
        let trigger = vcsv::from_field(&record, 2)?;
        let level = vcsv::from_option_field(&record, 4, 0)?;
        let gender = vcsv::from_option_field(&record, 5, Gender::Genderless)?;
        let move_id = vcsv::from_option_field(&record, 9, 0)?;
        let rps: VeekunOption<_> = vcsv::from_field(&record, 12)?;
        self.0.insert(species_id, EvolvesFrom {
            from_id: 0,
            trigger,
            level,
            gender,
            move_id,
            relative_physical_stats: rps.into(),
        });
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Species {
    pub name: String,
    pub generation: Generation,
    pub pokemon: Vec<Pokemon>,
    pub egg_groups: OneOrTwo<EggGroup>,
    pub evolves_from: Option<EvolvesFrom>,
}

pub struct SpeciesTable(Vec<Species>);

impl vcsv::FromCsvIncremental for SpeciesTable {
    fn from_empty_csv() -> Self {
        let default = Species {
            name: String::new(),
            generation: Generation::I,
            pokemon: Vec::new(),
            egg_groups: OneOrTwo::One(EggGroup::NoEggs),
            evolves_from: None,
        };
        SpeciesTable(repeat(default).take(POKEMON_COUNT).collect::<Vec<_>>())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let identifier: VeekunString = vcsv::from_field(&record, 1)?;
        let generation = vcsv::from_field(&record, 2)?;
        let i = id - 1;
        self.0[i].name = to_pascal_case(identifier.as_str());
        self.0[i].generation = generation;
        if let VeekunOption(Some(from_id)) = vcsv::from_field(&record, 3)? {
            self.0[i].evolves_from = Some(EvolvesFrom {
                from_id,
                trigger: EvolutionTrigger::LevelUp,
                level: 0,
                gender: Gender::Genderless,
                move_id: 0,
                relative_physical_stats: None,
            });
        }
        Ok(())
    }
}

impl SpeciesTable {
    pub fn new() -> Self {
        let pokemon_table = PokemonTable::new();
        let egg_group_table
            = EggGroupTable::from_csv_data(vdata::EGG_GROUPS).unwrap();
        let evolution_table
            = EvolutionTable::from_csv_data(vdata::EVOLUTION).unwrap();
        let mut species_table
            = SpeciesTable::from_csv_data(vdata::SPECIES).unwrap();
        species_table.set_pokemon(&pokemon_table);
        species_table.set_egg_groups(&egg_group_table);
        species_table.set_evolutions(&evolution_table);
        species_table
    }

    fn set_pokemon(&mut self, pokemon_table: &PokemonTable) {
        for i in 0..SPECIES_COUNT {
            self.0[i].pokemon = pokemon_table.0[i].clone();
        }
    }

    fn set_egg_groups(&mut self, egg_group_table: &EggGroupTable) {
        for i in 0..SPECIES_COUNT {
            let options = [
                egg_group_table.0[i].get(0).map(|g| *g),
                egg_group_table.0[i].get(1).map(|g| *g),
            ];
            self.0[i].egg_groups = OneOrTwo::from_options(options).unwrap();
        }
    }

    fn set_evolutions(&mut self, evolution_table: &EvolutionTable) {
        for i in 0..SPECIES_COUNT {
            self.0[i].evolves_from
                = self.0[i].evolves_from.map(|e| EvolvesFrom {
                    from_id: e.from_id,
                    .. evolution_table.0[&((i + 1) as u16)]
                });
        }
    }
}
