pub(self) mod ailments;
pub(self) mod flags;

pub use self::ailments::Ailment;
pub use self::flags::Flags;
pub use self::flags::FlagTable;

use std::collections::HashMap;
use crate::enums::*;
use crate::FromVeekun;
use crate::moves::MOVE_COUNT;
use crate::Stat;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use crate::VeekunOption;
use super::MoveId;

/// Broad move category.
#[EnumRepr(type = "u8")]
pub enum Category {
    /// Moves that inflict damage, potentially with some other minor effect.
    Damage = 0,
    /// Non-damaging moves that can inflict some ailment.
    Ailment,
    /// Non-damaging moves that adjust stats in favor of the user (raise user or
    /// lower target).
    NetGoodStats,
    /// Non-damaging moves that heal the user.
    Heal,
    /// Damaging moves that can inflict some ailment.
    DamageAilment,
    /// Non-damaging moves that confuse and raise a stat of the target.
    Swagger,
    /// Damaging moves that lower the target's stats.
    DamageLower,
    /// Damaging moves that raise the user's stats.
    DamageRaise,
    /// Damaging moves that heal the user half the damage inflicted.
    DamageHeal,
    /// Moves that cause a one-hit KO.
    OneHitKO,
    /// Non-damaging moves that affect the entire field.
    WholeFieldEffect,
    /// Non-damaging moves that affect half of the field.
    FieldEffect,
    /// Non-damaging moves that force the target to switch out.
    ForceSwitch,
    /// Moves that do fall into the other categories.
    Unique,
}

impl Default for Category {
    fn default() -> Self { Category::Unique }
}

impl FromVeekun for Category {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Category::from_repr(value)
    }
}

/// The number of stats directly changeable by moves (all but HP).
pub const CHANGEABLE_STATS: usize = 7;

#[derive(Default)]
pub struct StatChangeTable(pub HashMap<MoveId, [i8; CHANGEABLE_STATS]>);

impl StatChangeTable {
    pub fn new() -> Self {
        StatChangeTable::from_csv_data(vdata::MOVE_STAT_CHANGES).unwrap()
    }
}

impl vcsv::FromCsvIncremental for StatChangeTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: MoveId = vcsv::from_field(&record, 0)?;
        if id.0 >= 10000 {
            return Ok(())
        }
        let stat: Stat = vcsv::from_field(&record, 1)?;
        let change = vcsv::from_field(&record, 2)?;
        let mut stat_changes = self.0.get(&id)
            .map_or([0; CHANGEABLE_STATS], |v| *v);
        stat_changes[stat.repr() as usize] = change;
        self.0.insert(id, stat_changes);
        Ok(())
    }
}

/// Namespace for move data deemed "meta."
#[derive(Copy, Clone, Debug, Default)]
pub struct Meta {
    /// The move category.
    pub category: Category,
    /// The status ailment caused by the move.
    pub ailment: Ailment,
    /// If the move hits multiple times in one turn, the inclusive range for the
    /// number of hits.
    pub hits: Option<(u8, u8)>,
    /// If the move has an effect over several turns, the inclusive range for
    /// the number of turns.
    pub turns: Option<(u8, u8)>,
    /// The percent of damage absorbed (positive) or recoiled (negative).
    pub recoil: i8,
    /// The percent of max HP recovered (positive) or lost (negative).
    pub healing: i8,
    /// The increase of the critical rate when using the move.
    pub critical_rate: i8,
    /// The chance the move has of inflicting its ailment if it hits.
    pub ailment_chance: u8,
    /// The chance the move has of causing the target to flinch.
    pub flinch_chance: u8,
    /// The chance the move has of changing stats.
    pub stat_chance: u8,
    /// The changes the move can make to stats.
    pub stat_changes: [i8; CHANGEABLE_STATS],
    /// Move bitflags.
    pub flags: Flags,
}

pub struct MetaTable(pub [Meta; MOVE_COUNT]);

impl MetaTable {
    pub fn new() -> Self {
        let mut table = MetaTable::from_csv_data(vdata::MOVE_META).unwrap();
        table.set_flags(&FlagTable::new());
        table.set_stat_changes(&StatChangeTable::new());
        table
    }

    fn set_flags(&mut self, flags_table: &FlagTable) {
        for (id, flags) in flags_table.0.iter() {
            self[*id].flags = *flags;
        }
    }

    fn set_stat_changes(&mut self, stat_changes_table: &StatChangeTable) {
        for (id, stat_changes) in stat_changes_table.0.iter() {
            self[*id].stat_changes = *stat_changes;
        }
    }
}

impl Default for MetaTable {
    fn default() -> Self {
        MetaTable([Default::default(); MOVE_COUNT])
    }
}

impl vcsv::FromCsvIncremental for MetaTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: MoveId = vcsv::from_field(&record, 0)?;
        if id.0 >= 10000 {
            return Ok(())
        }
        let min_hits: VeekunOption<u8> = vcsv::from_field(&record, 3)?;
        let max_hits: VeekunOption<u8> = vcsv::from_field(&record, 4)?;
        let hits = match min_hits.into() {
            Some(min) => match max_hits.into() {
                Some(max) => Some((min, max)),
                None => None,
            },
            None => None,
        };
        let min_turns: VeekunOption<u8> = vcsv::from_field(&record, 5)?;
        let max_turns: VeekunOption<u8> = vcsv::from_field(&record, 6)?;
        let turns = match min_turns.into() {
            Some(min) => match max_turns.into() {
                Some(max) => Some((min, max)),
                None => None,
            },
            None => None,
        };
        self[id] = Meta {
            category: vcsv::from_field(&record, 1)?,
            ailment: vcsv::from_field(&record, 2)?,
            hits,
            turns,
            recoil: vcsv::from_field(&record, 7)?,
            healing: vcsv::from_field(&record, 8)?,
            critical_rate: vcsv::from_field(&record, 9)?,
            ailment_chance: vcsv::from_field(&record, 10)?,
            flinch_chance: vcsv::from_field(&record, 11)?,
            stat_chance: vcsv::from_field(&record, 12)?,
            stat_changes: [0; CHANGEABLE_STATS],
            flags: Flags::empty(),
        };
        Ok(())
    }
}

impl std::ops::Index<MoveId> for MetaTable {
    type Output = Meta;

    fn index(&self, index: MoveId) -> &Meta {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<MoveId> for MetaTable {
    fn index_mut(&mut self, index: MoveId) -> &mut Meta {
        self.0.index_mut(index.0 as usize)
    }
}
