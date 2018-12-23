use std::collections::HashMap;
use enums::*;
use FromVeekun;
use moves::CHANGEABLE_STATS;
use Stat;
use vcsv;
use veekun::repr::VeekunOption;

#[EnumRepr(type = "i8")]
pub enum Ailment {
    Unknown = -1,
    None,
    Paralysis,
    Sleep,
    Freeze,
    Burn,
    Poison,
    Confusion,
    Infatuation,
    Trap,
    Nightmare,
    Torment = 12,
    Disable,
    Yawn,
    HealBlock,
    NoTypeImmunity = 17,
    LeechSeed,
    Embargo,
    PerishSong,
    Ingrain,
}

#[EnumRepr(type = "u8")]
pub enum Category {
    Damage = 0,
    Ailment,
    NetGoodStats,
    Heal,
    DamageAilment,
    Swagger,
    DamageLower,
    DamageRaise,
    DamageHeal,
    OneHitKO,
    WholeFieldEffect,
    FieldEffect,
    ForceSwitch,
    Unique,
}

bitflags! {
    pub struct Flags: u16 {
        const Contact = 0x0001;
        const Charge = 0x0002;
        const Recharge = 0x0004;
        const Protect = 0x0008;
        const Reflectable = 0x0010;
        const Snatch = 0x0020;
        const Mirror = 0x0040;
        const Punch = 0x0080;
        const Sound = 0x0100;
        const Gravity = 0x0200;
        const Defrost = 0x0400;
        const Distance = 0x0800;
        const Heal = 0x1000;
        const Authentic = 0x2000;
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

pub struct Metadata {
    pub category: Category,
    pub ailment: Ailment,
    pub hits: Option<(u8, u8)>,
    pub turns: Option<(u8, u8)>,
    pub recoil: i8,
    pub healing: i8,
    pub critical_rate: i8,
    pub ailment_chance: u8,
    pub flinch_chance: u8,
    pub stat_chance: u8,
    pub stat_changes: [i8; CHANGEABLE_STATS],
    pub flags: Flags,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
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

pub struct MetaTable(pub HashMap<u16, Metadata>);

impl std::ops::Index<u16> for MetaTable {
    type Output = Metadata;

    fn index<'a>(&'a self, index: u16) -> &'a Metadata {
        self.0.index(&index)
    }
}

impl vcsv::FromCsvIncremental for MetaTable {
    fn from_empty_csv() -> Self {
        MetaTable(HashMap::new())
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id = vcsv::from_field(&record, 0)?;
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
        self.0.insert(id, Metadata {
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
        });
        Ok(())
    }
}

pub struct StatChangeTable(pub HashMap<u16, [i8; CHANGEABLE_STATS]>);

impl std::ops::Index<u16> for StatChangeTable {
    type Output = [i8; CHANGEABLE_STATS];

    fn index<'a>(&'a self, index: u16) -> &'a [i8; CHANGEABLE_STATS] {
        self.0.index(&index)
    }
}

impl vcsv::FromCsvIncremental for StatChangeTable {
    fn from_empty_csv() -> Self {
        StatChangeTable(HashMap::new())
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id = vcsv::from_field(&record, 0)?;
        let stat: Stat = vcsv::from_field(&record, 1)?;
        let change = vcsv::from_field(&record, 2)?;
        let mut stat_changes = self.0.get(&id)
            .map_or([0; CHANGEABLE_STATS], |v| *v);
        stat_changes[stat.repr() as usize] = change;
        self.0.insert(id, stat_changes);
        Ok(())
    }
}

pub struct FlagTable(pub HashMap<u16, Flags>);

impl std::ops::Index<u16> for FlagTable {
    type Output = Flags;

    fn index<'a>(&'a self, index: u16) -> &'a Flags {
        self.0.index(&index)
    }
}

impl vcsv::FromCsvIncremental for FlagTable {
    fn from_empty_csv() -> Self {
        FlagTable(HashMap::new())
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id = vcsv::from_field(&record, 0)?;
        let flag = vcsv::from_field(&record, 1)?;
        let new_flags = self.0.get(&id).map_or(flag, |v| flag | *v);
        self.0.insert(id, new_flags);
        Ok(())
    }
}
