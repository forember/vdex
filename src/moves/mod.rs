pub(self) mod effects;
pub(self) mod meta;

pub use self::effects::Effect;
pub use self::meta::Ailment;
pub use self::meta::Category;
pub use self::meta::Flags;

use std::collections::HashMap;
use enums::*;
use FromVeekun;
use to_pascal_case;
use Type;
use vcsv;
use veekun::repr::VeekunOption;
use versions::Generation;

pub const CHANGEABLE_STATS: usize = 7;

#[EnumRepr(type = "u8")]
pub enum BattleStyle {
    Attack = 1,
    Defense,
    Support,
}

#[EnumRepr(type = "u8")]
pub enum DamageClass {
    NonDamaging = 1,
    Physical,
    Special,
}

#[EnumRepr(type = "u8")]
pub enum LearnMethod {
    LevelUp = 1,
    Egg,
    Tutor,
    Machine,
    StadiumSurfingPikachu,
    LightBallEgg,
    ColosseumPurification,
    XDShadow,
    XDPurification,
    FormChange,
}

#[EnumRepr(type = "u8")]
pub enum Target {
    SpecificMove = 1,
    SelectedPokemonReuseStolen,
    Ally,
    UsersField,
    UserOrAlly,
    OpponentsField,
    User,
    RandomOpponent,
    AllOtherPokemon,
    SelectedPokemon,
    AllOpponents,
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

pub struct Move {
    pub id: u16,
    pub name: String,
    pub generation: Generation,
    pub typ: Type,
    pub power: u8,
    pub pp: u8,
    pub accuracy: u8,
    pub priority: i8,
    pub target: Target,
    pub damage_class: DamageClass,
    pub effect: Effect,
    pub effect_chance: Option<u8>,
    pub meta: meta::Metadata,
}

pub struct MoveTable(pub HashMap<u16, Move>);

impl MoveTable {
}

impl std::ops::Index<u16> for MoveTable {
    type Output = Move;

    fn index<'a>(&'a self, index: u16) -> &'a Move {
        self.0.index(&index)
    }
}

impl vcsv::FromCsvIncremental for MoveTable {
    fn from_empty_csv() -> Self {
        MoveTable(HashMap::new())
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id = vcsv::from_field(&record, 0)?;
        let effect_chance: VeekunOption<_> = vcsv::from_field(&record, 11)?;
        self.0.insert(id, Move {
            id,
            name: to_pascal_case(vcsv::get_field(&record, 1)?),
            generation: vcsv::from_field(&record, 2)?,
            typ: vcsv::from_field(&record, 3)?,
            power: vcsv::from_field(&record, 4)?,
            pp: vcsv::from_field(&record, 5)?,
            accuracy: vcsv::from_field(&record, 6)?,
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
