//! Moves and related data.

pub(self) mod effects;
pub(self) mod meta;

pub use self::effects::Effect;
pub use self::meta::Ailment;
pub use self::meta::Category;
pub use self::meta::Flags;
pub use self::meta::Meta;

use enums::*;
use FromVeekun;
use to_pascal_case;
use Type;
use vcsv;
use vcsv::FromCsv;
use vdata;
use VeekunOption;
use versions::Generation;

/// The number of stats directly changeable by moves (all but HP).
pub const CHANGEABLE_STATS: usize = 7;
/// The total number of moves in pbirch.
pub const MOVE_COUNT: usize = 559;

/// The Battle Palace style of a move.
#[EnumRepr(type = "u8")]
pub enum BattleStyle {
    Attack = 1,
    Defense,
    Support,
}

/// The damage class (status, physical, or special) of a move.
#[EnumRepr(type = "u8")]
pub enum DamageClass {
    NonDamaging = 1,
    Physical,
    Special,
}

/// The method by which a Pokémon learns a move.
#[EnumRepr(type = "u8")]
pub enum LearnMethod {
    /// Learned at a certain level.
    LevelUp = 1,
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

/// The target selection mechanism of a move.
#[EnumRepr(type = "u8")]
pub enum Target {
    /// Target depends on some battle state (Counter, Curse, Mirror Coat, and
    /// Metal Burst).
    SpecificMove = 1,
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

impl FromVeekun for BattleStyle {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for DamageClass {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for LearnMethod {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

impl FromVeekun for Target {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

/// A move is the primary action that a Pokémon can take on its turn.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Move) A
/// > move (Japanese: わざ move), also known as an attack (Japanese:
/// > こうげきわざ attack technique) or technique (Japanese: とくしゅわざ
/// > special technique), is the skill Pokémon primarily use in battle. In
/// > battle, a Pokémon uses one move each turn.
#[derive(Debug)]
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

impl Default for Move {
    fn default() -> Self {
        Move {
            name: Default::default(),
            generation: Generation::I,
            typ: Type::Normal,
            power: 0,
            pp: 0,
            accuracy: None,
            priority: 0,
            target: Target::SelectedPokemon,
            damage_class: DamageClass::NonDamaging,
            effect: Effect::Splash,
            effect_chance: None,
            meta: Default::default(),
        }
    }
}

/// Wrapper of a `Vec` for all moves.
///
/// An move's index is its Veekun ID minus 1.
///
/// Use `table.0` to access `Vec` members.
pub struct MoveTable(pub Vec<Move>);

impl MoveTable {
    /// Create a move table from the included Veekun CSV data.
    pub fn new() -> Self {
        let meta_table = meta::MetaTable::new();
        let mut moves_table = MoveTable::from_csv_data(vdata::MOVES).unwrap();
        moves_table.set_meta(&meta_table);
        moves_table
    }

    fn set_meta(&mut self, meta_table: &meta::MetaTable) {
        for i in 0..MOVE_COUNT {
            self.0[i].meta = meta_table.0[i];
        }
    }
}

impl vcsv::FromCsvIncremental for MoveTable {
    fn from_empty_csv() -> Self {
        MoveTable(vec![])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        if id > 10000 {
            return Ok(())
        }
        let accuracy: VeekunOption<_> = vcsv::from_field(&record, 6)?;
        let effect_chance: VeekunOption<_> = vcsv::from_field(&record, 11)?;
        self.0.push(Move {
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
        });
        Ok(())
    }
}
