use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use enums::*;
use FromVeekun;
use moves::{CHANGEABLE_STATS, MOVE_COUNT};
use Stat;
use vcsv;
use vcsv::FromCsv;
use VeekunOption;

/// Aka status condition; an ailment caused by a move.
///
/// > [*[From
/// > Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Status_condition)
/// > Status conditions (Japanese: 状態異常 abnormal condition), also referred
/// > to as status problems or status ailments, affect a Pokémon's ability to
/// > battle. There are three kinds of status. The first are non-volatile, the
/// > second are volatile, and the third lasts while a Pokémon is in battle. The
/// > Pokérus is a similar but unrelated concept.
#[EnumRepr(type = "i8")]
pub enum Ailment {
    /// Some special ailment: used by Tri Attack, Telekinesis, and Smack Down.
    Unknown = -1,
    /// No ailment.
    None,
    /// A paralyzed Pokémon has a 25% chance of not being able to move, and its
    /// speed is decreased to 25% of its normal value.
    Paralysis,
    /// A sleeping Pokémon cannot move. Sleep normally lasts 2-5 turns.
    Sleep,
    /// A frozen Pokémon cannot move while frozen, but has a 20% chance of
    /// thawing each time it attempts to move.
    Freeze,
    /// A burned Pokémon takes 1/8 of its max HP at the end of each turn, and
    /// damage it does with phyisical moves is halved.
    Burn,
    /// A poisoned Pokémon takes 1/8 of its max HP at the end of each turn.
    ///
    /// A *badly* poisoned Pokémon takes *n*/16 of its max HP at the end of each
    /// turn, where *n* is the number of turns since the poisoning, starting at
    /// 1\. If a badly poisoned Pokémon is switched out, *n* resets to 1. At the
    /// end of a battle, bad poisoning becomes regular poisoning.
    Poison,
    /// A confused Pokémon will sometimes hurt itself when attempting to attack.
    ///
    /// > [*[From
    /// > Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Status_condition#Confusion)
    /// > The confused condition causes a Pokémon to sometimes hurt itself in
    /// > its confusion instead of executing a selected move.  [The chance to
    /// > hurt itself is 50%.] The damage is done as if the Pokémon attacked
    /// > itself with a 40-power typeless physical attack (without the
    /// > possibility of a critical hit).
    /// >
    /// > Confusion wears off after 1-4 attacking turns. This means that turns
    /// > recharging, such as after using Hyper Beam, and turns unable to
    /// > attack, such as from paralysis, will not lower the remaining number of
    /// > turns of confusion. However, a sleeping Pokémon may hurt itself in
    /// > confusion if using a move such as Snore or Sleep Talk. Multi-turn
    /// > attacks such as Fly and Dive require confusion to be checked both
    /// > turns, further reducing the chance of a successful attack.
    Confusion,
    /// An infatuated Pokémon has a 50% chance of not being able to attack.
    Infatuation,
    /// A trapped Pokémon cannot switch out.
    Trap,
    /// A Pokémon under the effect of Nightmare takes 1/4 of its max HP at the
    /// end of each turn. The nightmare ends when the Pokémon wakes up.
    Nightmare,
    /// A tormented Pokémon cannot use the same move consecutively.
    Torment = 12,
    /// A Pokémon cannot use a move that has been disabled. Disable lasts 4
    /// turns.
    Disable,
    /// A drowsy Pokémon will fall asleep next turn if it does not switch out.
    Yawn,
    /// A Pokémon under the effect of Heal Block cannot heal for five turns.
    HealBlock,
    /// One of the Pokémon's types (Ghost or Dark) has its immunities disabled.
    NoTypeImmunity = 17,
    /// A seeded Pokémon takes 1/8 of its max HP at the end of each turn,
    /// healing the Pokémon in the position of the seeder the same amount.
    LeechSeed,
    /// A embargoed Pokémon cannot use items for 5 turns.
    Embargo,
    /// A Pokémon under the effect of Perish Song will faint in three turns
    /// unless switched out.
    PerishSong,
    /// A rooted Pokémon restores 1/16 of its max HP at the end of each turn,
    /// but cannot switch out.
    Ingrain,
}

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

bitflags! {
    /// Miscellaneous bitflags for moves.
    pub struct Flags: u16 {
        /// The move makes contact with the target.
        const Contact = 0x0001;
        /// The move requires a turn to charge before attacking.
        const Charge = 0x0002;
        /// The move requires a turn to recharge after attacking.
        const Recharge = 0x0004;
        /// The move is blocked by Detect and Protect.
        const Protect = 0x0008;
        /// The move is reflected by Magic Coat and Magic Bounce.
        const Reflectable = 0x0010;
        /// The move is stolen by Snatch.
        const Snatch = 0x0020;
        /// The move is copied by Mirror Move.
        const Mirror = 0x0040;
        /// The move is boosted by Iron Fist.
        const Punch = 0x0080;
        /// The move is blocked by Soundproof.
        const Sound = 0x0100;
        /// The move is unusable under Gravity.
        const Gravity = 0x0200;
        /// The move can be used while frozen to thaw.
        const Defrost = 0x0400;
        /// Affects triple battles; unused in pbirch.
        const Distance = 0x0800;
        /// The move is blocked by Heal Block.
        const Heal = 0x1000;
        /// The move ignores Substitute.
        const Authentic = 0x2000;
    }
}

impl Ailment {
    /// True if the ailment does not persist on switching out.
    pub fn volatile(self) -> bool {
        match self.repr() {
            1 ... 5 => false,
            _ => true,
        }
    }
}

impl FromVeekun for Ailment {
    type Intermediate = i8;

    fn from_veekun(value: i8) -> Option<Self> {
        Ailment::from_repr(value)
    }
}

impl FromVeekun for Category {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        Category::from_repr(value)
    }
}

impl FromVeekun for Flags {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
            1 ... 14 => Flags::from_bits(1 << (value - 1)),
            _ => None,
        }
    }
}

/// Namespace for move data deemed "meta."
#[derive(Copy, Clone, Debug)]
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

impl Default for Meta {
    fn default() -> Self {
        Meta {
            category: Category::Damage,
            ailment: Ailment::Unknown,
            hits: None,
            turns: None,
            recoil: 0,
            healing: 0,
            critical_rate: 0,
            ailment_chance: 0,
            flinch_chance: 0,
            stat_chance: 0,
            stat_changes: [0; CHANGEABLE_STATS],
            flags: Flags::empty(),
        }
    }
}

pub struct MetaTable(pub [Meta; MOVE_COUNT]);

impl MetaTable {
    pub fn from_files<S: AsRef<OsStr> + ?Sized>(
        meta_file: &S, stat_changes_file: &S, flags_file: &S
    ) -> vcsv::Result<Self> {
        let flags_path = Path::new(flags_file);
        let flags_table = FlagTable::from_csv_file(flags_path)?;
        let stat_changes_path = Path::new(stat_changes_file);
        let stat_changes_table
            = StatChangeTable::from_csv_file(stat_changes_path)?;
        let meta_path = Path::new(meta_file);
        let mut meta_table = MetaTable::from_csv_file(meta_path)?;
        meta_table.set_stat_changes(&stat_changes_table);
        meta_table.set_flags(&flags_table);
        Ok(meta_table)
    }

    fn set_stat_changes(&mut self, stat_changes_table: &StatChangeTable) {
        for (id, stat_changes) in stat_changes_table.0.iter() {
            self.0[id - 1].stat_changes = *stat_changes;
        }
    }

    fn set_flags(&mut self, flags_table: &FlagTable) {
        for (id, flags) in flags_table.0.iter() {
            self.0[id - 1].flags = *flags;
        }
    }
}

impl vcsv::FromCsvIncremental for MetaTable {
    fn from_empty_csv() -> Self {
        MetaTable([Default::default(); MOVE_COUNT])
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        if id > 10000 {
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
        self.0[id - 1] = Meta {
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

pub struct StatChangeTable(pub HashMap<usize, [i8; CHANGEABLE_STATS]>);

impl vcsv::FromCsvIncremental for StatChangeTable {
    fn from_empty_csv() -> Self {
        StatChangeTable(HashMap::new())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        if id > 10000 {
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

pub struct FlagTable(pub HashMap<usize, Flags>);

impl vcsv::FromCsvIncremental for FlagTable {
    fn from_empty_csv() -> Self {
        FlagTable(HashMap::new())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        if id > 10000 {
            return Ok(())
        }
        let flag = vcsv::from_field(&record, 1)?;
        let new_flags = self.0.get(&id).map_or(flag, |v| flag | *v);
        self.0.insert(id, new_flags);
        Ok(())
    }
}
