//! Pokemon and related data.

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

impl Default for EggGroup {
    fn default() -> Self { EggGroup::NoEggs }
}

impl FromVeekun for EggGroup {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

/// The method by which a Pokémon evolves.
#[EnumRepr(type = "u8")]
pub enum EvolutionTrigger {
    LevelUp = 1,
    Trade,
    UseItem,
    Shed,
}

impl Default for EvolutionTrigger {
    fn default() -> Self { EvolutionTable::Trade }
}

impl FromVeekun for EvolutionTrigger {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
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

impl Default for Gender {
    fn default() -> Self { Gender::Genderless }
}

impl FromVeekun for Gender {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
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

impl<T: Copy + Default> Default for OneOrTwo<T> {
    fn default() -> Self { OneOrTwo::One(Default::default()) }
}

const POKEMON_COUNT: usize = 673;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PokemonId(pub u16);

impl Default for PokemonId {
    fn default() -> Self { PokemonId(std::u16::MAX) }
}

impl FromVeekun for PokemonId {
    type Intermediate = u16;

    fn from_veekun(value: u16) -> Option<Self> {
        let id = value.checked_sub(1).unwrap_or_default();
        if id < (POKEMON_COUNT as u16) {
            Some(PokemonId(id))
        } else {
            None
        }
    }
}

#[derive(Default)]
struct AbilityTable([[Option<Ability>; 3]; POKEMON_COUNT]);

impl AbilityTable {
    fn new() -> Self {
        AbilityTable::from_csv_data(vdata::ABILITIES).unwrap()
    }
}

impl vcsv::FromCsvIncremental for AbilityTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: PokemonId = vcsv::from_field(&record, 0)?;
        let ability = vcsv::from_field(&record, 1)?;
        let slot: usize = vcsv::from_field(&record, 3)?;
        if slot < 1 && slot > 3 {
            return Err(vcsv::Error::Veekun {
                line: record.position(),
                field: 3,
                error: Box::new(vcsv::MiscError("Invalid slot number")),
            });
        }
        self[id][slot - 1] = Some(ability);
        Ok(())
    }
}

impl std::ops::Index<PokemonId> for AbilityTable {
    type Output = [Option<Ability>; 3];

    fn index(&self, index: PokemonId) -> &[Option<Ability>; 3] {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<PokemonId> for AbilityTable {
    fn index_mut(&mut self, index: PokemonId) -> &mut [Option<Ability>; 3] {
        self.0.index_mut(index.0 as usize)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Form {
    pub id: u16,
    pub name: Option<String>,
    pub battle_only: bool,
}

struct FormTable(Vec<Vec<Form>>);

impl FormTable {
    fn new() -> Self {
        FormTable::from_csv_data(vdata::FORMS).unwrap()
    }
}

impl Default for FormTable {
    fn default() -> Self {
        FormTable(repeat(Vec::new()).take(POKEMON_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for FormTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let form_id = vcsv::from_field(&record, 0)?;
        let name: VeekunOption<VeekunString> = vcsv::from_field(&record, 1)?;
        let pokemon_id: PokemonId = vcsv::from_field(&record, 2)?;
        let battle_only: u8 = vcsv::from_field(&record, 5)?;
        self[pokemon_id].push(Form {
            id: form_id,
            name: name.into(),
            battle_only: battle_only != 0,
        });
        Ok(())
    }
}

impl std::ops::Index<PokemonId> for FormTable {
    type Output = Vec<Form>;

    fn index(&self, index: PokemonId) -> &Vec<Form> {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<PokemonId> for FormTable {
    fn index_mut(&mut self, index: PokemonId) -> &mut Vec<Form> {
        self.0.index_mut(index.0 as usize)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PokemonMove {
    pub move_id: MoveId,
    pub learn_method: LearnMethod,
    pub level: u8,
}

struct PokemonMoveTable(Vec<HashMap<VersionGroup, Vec<PokemonMove>>>);

impl PokemonMoveTable {
    fn new() -> Self {
        PokemonMoveTable::from_csv_data(vdata::POKEMON_MOVES).unwrap()
    }
}

impl Default for PokemonMoveTable {
    fn default() -> Self {
        PokemonMoveTable(repeat(HashMap::new())
                .take(POKEMON_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for PokemonMoveTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: PokemonId = vcsv::from_field(&record, 0)?;
        let version_group = vcsv::from_field(&record, 1)?;
        let move_id = vcsv::from_field(&record, 2)?;
        let learn_method = vcsv::from_field(&record, 3)?;
        let level = vcsv::from_field(&record, 4)?;
        let pokemon_move = PokemonMove { move_id, learn_method, level };
        self[pokemon_id].entry(version_group)
            .or_insert(Vec::new()).push(pokemon_move);
        Ok(())
    }
}

impl std::ops::Index<PokemonId> for FormTable {
    type Output = Vec<Form>;

    fn index(&self, index: PokemonId) -> &Vec<Form> {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<PokemonId> for FormTable {
    fn index_mut(&mut self, index: PokemonId) -> &mut Vec<Form> {
        self.0.index_mut(index.0 as usize)
    }
}

/// The number of stats that exist out of battle (all but accuracy and evasion).
pub const PERMANENT_STATS: usize = 6;

/// A Pokémon's base permanent stats.
#[derive(Copy, Clone, Debug, Default)]
pub struct BaseStats(pub [u8; PERMANENT_STATS]);

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

#[derive(Default)]
struct StatTable([BaseStats; POKEMON_COUNT]);

impl StatTable {
    fn new() -> Self {
        StatTable::from_csv_data(vdata::STATS).unwrap()
    }
}

impl vcsv::FromCsvIncremental for StatTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: PokemonId = vcsv::from_field(&record, 0)?;
        let stat = vcsv::from_field(&record, 1)?;
        let base = vcsv::from_field(&record, 2)?;
        self[id][stat] = base;
        Ok(())
    }
}

impl std::ops::Index<PokemonId> for StatTable {
    type Output = BaseStats;

    fn index(&self, index: PokemonId) -> &BaseStats {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<PokemonId> for StatTable {
    fn index_mut(&mut self, index: PokemonId) -> &mut BaseStats {
        self.0.index_mut(index.0 as usize)
    }
}

#[derive(Default)]
struct TypeTable([[Option<Type>; 2]; POKEMON_COUNT]);

impl TypeTable {
    fn new() -> Self {
        TypeTable::from_csv_data(vdata::TYPES).unwrap()
    }
}

impl vcsv::FromCsvIncremental for TypeTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: PokemonId = vcsv::from_field(&record, 0)?;
        let typ = vcsv::from_field(&record, 1)?;
        let slot: usize = vcsv::from_field(&record, 2)?;
        if slot < 1 && slot > 2 {
            return Err(vcsv::Error::Veekun {
                line: record.position(),
                field: 2,
                error: Box::new(vcsv::MiscError("Invalid slot number")),
            });
        }
        self.0[id][slot - 1] = Some(typ);
        Ok(())
    }
}

/// The total number of Pokémon species in pbirch.
pub const SPECIES_COUNT: usize = 649;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SpeciesId(pub u16);

impl Default for SpeciesId {
    fn default() -> Self { SpeciesId(std::u16::MAX) }
}

impl FromVeekun for SpeciesId {
    type Intermediate = u16;

    fn from_veekun(value: u16) -> Option<Self> {
        let id = value.checked_sub(1).unwrap_or_default();
        if id < (SPECIES_COUNT as u16) {
            Some(SpeciesId(id))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Pokemon {
    pub id: PokemonId,
    pub abilities: OneOrTwo<Ability>,
    pub hidden_ability: Option<Ability>,
    pub forms: Vec<Form>,
    pub moves: HashMap<VersionGroup, Vec<PokemonMove>>,
    pub stats: BaseStats,
    pub types: OneOrTwo<Type>,
}

struct PokemonTable(Vec<Vec<Pokemon>>);

impl Default for PokemonTable {
    fn default() -> Self {
        PokemonTable(repeat(Vec::new()).take(SPECIES_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for PokemonTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let pokemon_id: PokemonId = vcsv::from_field(&record, 0)?;
        let species_id: SpeciesId = vcsv::from_field(&record, 1)?;
        self[species_id].push(Pokemon {
            id: pokemon_id,
            .. Default::default()
        });
        Ok(())
    }
}

impl std::ops::Index<SpeciesId> for PokemonTable {
    type Output = Vec<Pokemon>;

    fn index(&self, index: SpeciesId) -> &Vec<Pokemon> {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<SpeciesId> for PokemonTable {
    fn index_mut(&mut self, index: SpeciesId) -> &mut Vec<Pokemon> {
        self.0.index_mut(index.0 as usize)
    }
}

impl PokemonTable {
    fn new() -> Self {
        let mut table = PokemonTable::from_csv_data(vdata::POKEMON).unwrap();
        table.set_abilities(&AbilityTable::new());
        table.set_forms(&FormTable::new());
        table.set_moves(&PokemonMoveTable::new());
        table.set_types(&TypeTable::new());
        table.set_stats(&StatTable::new());
        table
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

impl EggGroupTable {
    fn new() -> Self {
        EggGroupTable::from_csv_data(vdata::EGG_GROUPS).unwrap()
    }
}

impl Default for EggGroupTable {
    fn default() -> Self {
        EggGroupTable(repeat(Vec::new()).take(SPECIES_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for EggGroupTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: SpeciesId = vcsv::from_field(&record, 0)?;
        let egg_group = vcsv::from_field(&record, 1)?;
        self[id].push(egg_group);
        Ok(())
    }
}

impl std::ops::Index<SpeciesId> for PokemonTable {
    type Output = Vec<EggGroup>;

    fn index(&self, index: SpeciesId) -> &Vec<EggGroup> {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<SpeciesId> for PokemonTable {
    fn index_mut(&mut self, index: SpeciesId) -> &mut Vec<EggGroup> {
        self.0.index_mut(index.0 as usize)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EvolvesFrom {
    pub from_id: SpeciesId,
    pub trigger: EvolutionTrigger,
    pub level: u8,
    pub gender: Gender,
    pub move_id: u16,
    pub relative_physical_stats: Option<i8>,
}

#[derive(Default)]
struct EvolutionTable(HashMap<SpeciesId, EvolvesFrom>);

impl EvolutionTable {
    fn new() -> Self {
        EvolutionTable::from_csv_data(vdata::EVOLUTION).unwrap()
    }
}

impl vcsv::FromCsvIncremental for EvolutionTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let species_id = vcsv::from_field(&record, 1)?;
        let trigger = vcsv::from_field(&record, 2)?;
        let level = vcsv::from_option_field(&record, 4, 0)?;
        let gender = vcsv::from_option_field(&record, 5, Gender::Genderless)?;
        let move_id = vcsv::from_option_field(&record, 9, 0)?;
        let rps: VeekunOption<_> = vcsv::from_field(&record, 12)?;
        self.0.insert(species_id, EvolvesFrom {
            from_id: Default::default(),
            trigger,
            level,
            gender,
            move_id,
            relative_physical_stats: rps.into(),
        });
        Ok(())
    }
}

impl std::ops::Index<SpeciesId> for EvolutionTable {
    type Output = EvolvesFrom;

    fn index(&self, index: SpeciesId) -> &EvolvesFrom {
        self.0.index(index.0 as usize)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Species {
    pub name: String,
    pub generation: Generation,
    pub pokemon: Vec<Pokemon>,
    pub egg_groups: OneOrTwo<EggGroup>,
    pub evolves_from: Option<EvolvesFrom>,
}

pub struct SpeciesTable(Vec<Species>);

impl Default {
    fn default() -> Self {
        SpeciesTable(repeat(Default::default())
                .take(SPECIES_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for SpeciesTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: SpeciesId = vcsv::from_field(&record, 0)?;
        let identifier: VeekunString = vcsv::from_field(&record, 1)?;
        let generation = vcsv::from_field(&record, 2)?;
        self[id].name = to_pascal_case(identifier.as_str());
        self[id].generation = generation;
        if let VeekunOption(Some(from_id)) = vcsv::from_field(&record, 3)? {
            self[id].evolves_from = Some(EvolvesFrom {
                from_id,
                .. Default::default()
            });
        }
        Ok(())
    }
}

impl std::ops::Index<SpeciesId> for PokemonTable {
    type Output = Species;

    fn index(&self, index: SpeciesId) -> &Species {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<SpeciesId> for PokemonTable {
    fn index_mut(&mut self, index: SpeciesId) -> &mut Species {
        self.0.index_mut(index.0 as usize)
    }
}

impl SpeciesTable {
    pub fn new() -> Self {
        let mut table = SpeciesTable::from_csv_data(vdata::SPECIES).unwrap();
        table.set_pokemon(&PokemonTable::new());
        table.set_egg_groups(&EggGroupTable::new());
        table.set_evolutions(&EvolutionTable::new());
        table
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
