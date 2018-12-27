pub(self) mod effects;
pub(self) mod meta;

pub use self::effects::Effect;
pub use self::meta::Ailment;
pub use self::meta::Category;
pub use self::meta::Flags;
pub use self::meta::Metadata;

use std::path::Path;
use std::ffi::OsStr;
use enums::*;
use FromVeekun;
use to_pascal_case;
use Type;
use vcsv;
use vcsv::FromCsv;
use VeekunOption;
use versions::Generation;

pub const CHANGEABLE_STATS: usize = 7;
pub const MOVE_COUNT: usize = 559;

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

#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub generation: Generation,
    pub typ: Type,
    pub power: u8,
    pub pp: u8,
    pub accuracy: Option<u8>,
    pub priority: i8,
    pub target: Target,
    pub damage_class: DamageClass,
    pub effect: Effect,
    pub effect_chance: Option<u8>,
    pub meta: meta::Metadata,
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

pub struct MoveTable(pub Vec<Move>);

impl MoveTable {
    pub fn from_files<S: AsRef<OsStr> + ?Sized>(
        moves_file: &S, meta_file: &S, stat_changes_file: &S, flags_file: &S
    ) -> vcsv::Result<Self> {
        let meta_table = meta::MetaTable::from_files(
            meta_file, stat_changes_file, flags_file)?;
        let moves_path = Path::new(moves_file);
        let mut moves_table = MoveTable::from_csv_file(moves_path)?;
        moves_table.set_meta(&meta_table);
        Ok(moves_table)
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
