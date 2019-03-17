//! Moves and related data.

pub(self) mod effects;
pub(self) mod meta;

pub use self::effects::Effect;
pub use self::meta::Ailment;
pub use self::meta::Category;
pub use self::meta::Flags;
pub use self::meta::Meta;

use std::iter::repeat;
use crate::enums::*;
use crate::FromVeekun;
use crate::to_pascal_case;
use crate::Type;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use crate::VeekunOption;
use crate::versions::Generation;

/// The Battle Palace style of a move.
#[EnumRepr(type = "u8")]
pub enum BattleStyle {
    Attack = 0,
    Defense,
    Support,
}

impl Default for BattleStyle {
    fn default() -> Self { BattleStyle::Support }
}

impl FromVeekun for BattleStyle {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

/// The damage class (status, physical, or special) of a move.
#[EnumRepr(type = "u8")]
pub enum DamageClass {
    NonDamaging = 0,
    Physical,
    Special,
}

impl Default for BattleStyle {
    fn default() -> Self { DamageClass::NonDamaging }
}

impl FromVeekun for DamageClass {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

/// The method by which a Pokémon learns a move.
#[EnumRepr(type = "u8")]
pub enum LearnMethod {
    /// Learned at a certain level.
    LevelUp = 0,
    /// Known by newly-hatched Pokémon if the father knew it.
    Egg,
    /// Taught by a move tutor.
    Tutor,
    /// Taught using a TM or HM.
    Machine,
    /// Stadium; unused in pbirch.
    StadiumSurfingPikachu,
    /// Known by newly-hatched Pichu is mother was holding a Light Ball.
    LightBallEgg,
    /// Shadow; unused in pbirch.
    ColosseumPurification,
    /// Shadow; unused in pbirch.
    XDShadow,
    /// Shadow; unused in pbirch.
    XDPurification,
    /// Appears via Rotom form change.
    FormChange,
}

impl Default for BattleStyle {
    fn default() -> Self { LearnMethod::LevelUp }
}

impl FromVeekun for LearnMethod {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

/// The target selection mechanism of a move.
#[EnumRepr(type = "u8")]
pub enum Target {
    /// Target depends on some battle state (Counter, Curse, Mirror Coat, and
    /// Metal Burst).
    SpecificMove = 0,
    /// One selected Pokémon (not the user). Stolen moves reuse the same target.
    SelectedPokemonReuseStolen,
    /// The user's ally (Helping Hand).
    Ally,
    /// The user side of the field (user and ally).
    UsersField,
    /// Selected user or ally (Acupressure).
    UserOrAlly,
    /// The opposing side of the field (Spikes, Toxic Spikes, and Stealth Rock).
    OpponentsField,
    /// The user.
    User,
    /// One random opposing Pokémon.
    RandomOpponent,
    /// All Pokémon other than the user.
    AllOtherPokemon,
    /// One selected Pokémon (not the user).
    SelectedPokemon,
    /// All opposing Pokémon.
    AllOpponents,
    /// The entire field.
    EntireField,
}

impl Default for BattleStyle {
    fn default() -> Self { LearnMethod::SpecificMove }
}

impl FromVeekun for Target {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

/// The total number of moves in pbirch.
pub const MOVE_COUNT: usize = 559;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct MoveId(pub u16);

impl Default for MoveId {
    fn default() -> Self { MoveId(std::u16::MAX) }
}

impl FromVeekun for MoveId {
    type Intermediate = u16;

    fn from_veekun(value: u16) -> Option<Self> {
        let id = value.checked_sub(1).unwrap_or_default();
        if id < (MOVE_COUNT as u16) {
            Some(MoveId(id))
        } else {
            None
        }
    }
}

/// A move is the primary action that a Pokémon can take on its turn.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Move) A
/// > move (Japanese: わざ move), also known as an attack (Japanese:
/// > こうげきわざ attack technique) or technique (Japanese: とくしゅわざ
/// > special technique), is the skill Pokémon primarily use in battle. In
/// > battle, a Pokémon uses one move each turn.
#[derive(Clone, Debug, Default)]
pub struct Move {
    /// The pbirch name for the move.
    pub name: String,
    /// The generation the move was introduced.
    pub generation: Generation,
    /// The move's type.
    pub typ: Type,
    /// The move's power.
    pub power: u8,
    /// The move's power points.
    pub pp: u8,
    /// The move's accuracy, or `None` if it cannot miss.
    pub accuracy: Option<u8>,
    /// The move's priority.
    pub priority: i8,
    /// The move's targeting mechanism.
    pub target: Target,
    /// The move's damage class.
    pub damage_class: DamageClass,
    /// The move's effect.
    pub effect: Effect,
    /// The move's effect chance, if relevant.
    pub effect_chance: Option<u8>,
    /// The move's "meta" data.
    pub meta: meta::Meta,
}

/// Wrapper of a `Vec` for all moves.
///
/// A move's index is its Veekun ID minus 1.
///
/// Use `table.0` to access `Vec` members.
pub struct MoveTable(pub Vec<Move>);

impl MoveTable {
    /// Create a move table from the included Veekun CSV data.
    pub fn new() -> Self {
        let mut table = MoveTable::from_csv_data(vdata::MOVES).unwrap();
        table.set_meta(&meta::MetaTable::new());
        table
    }

    fn set_meta(&mut self, meta_table: &meta::MetaTable) {
        for i in 0..MOVE_COUNT {
            self.0[i].meta = meta_table.0[i];
        }
    }
}

impl Default for MoveTable {
    fn default() -> Self {
        MoveTable(repeat(Vec::new()).take(MOVE_COUNT).collect::<Vec<_>>())
    }
}

impl vcsv::FromCsvIncremental for MoveTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: MoveId = vcsv::from_field(&record, 0)?;
        if id.0 >= 10000 {
            return Ok(())
        }
        let accuracy: VeekunOption<_> = vcsv::from_field(&record, 6)?;
        let effect_chance: VeekunOption<_> = vcsv::from_field(&record, 11)?;
        self[id] = Move {
            name: to_pascal_case(vcsv::get_field(&record, 1)?),
            generation: vcsv::from_field(&record, 2)?,
            typ: vcsv::from_field(&record, 3)?,
            power: vcsv::from_field(&record, 4)?,
            pp: vcsv::from_option_field(&record, 5, 0)?,
            accuracy: accuracy.into(),
            priority: vcsv::from_field(&record, 7)?,
            target: vcsv::from_field(&record, 8)?,
            damage_class: vcsv::from_field(&record, 9)?,
            effect: vcsv::from_field(&record, 10)?,
            effect_chance: effect_chance.into(),
            meta: Default::default(),
        };
        Ok(())
    }
}

impl std::ops::Index<MoveId> for MoveTable {
    type Output = Move;

    fn index(&self, index: MoveId) -> &Move {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<MoveId> for MoveTable {
    fn index_mut(&mut self, index: MoveId) -> &mut Move {
        self.0.index_mut(index.0 as usize)
    }
}
