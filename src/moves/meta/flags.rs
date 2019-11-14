use std::collections::HashMap;
use crate::FromVeekun;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;
use super::MoveId;

bitflags! {
    /// Miscellaneous bitflags for moves.
    pub struct Flags: u16 {
        /// The move makes contact with the target.
        const CONTACT = 0x0001;
        /// The move requires a turn to charge before attacking.
        const CHARGE = 0x0002;
        /// The move requires a turn to recharge after attacking.
        const RECHARGE = 0x0004;
        /// The move is blocked by Detect and Protect.
        const PROTECT = 0x0008;
        /// The move is reflected by Magic Coat and Magic Bounce.
        const REFLECTABLE = 0x0010;
        /// The move is stolen by Snatch.
        const SNATCH = 0x0020;
        /// The move is copied by Mirror Move.
        const MIRROR = 0x0040;
        /// The move is boosted by Iron Fist.
        const PUNCH = 0x0080;
        /// The move is blocked by Soundproof.
        const SOUND = 0x0100;
        /// The move is unusable under Gravity.
        const GRAVITY = 0x0200;
        /// The move can be used while frozen to thaw.
        const DEFROST = 0x0400;
        /// Affects triple battles; unused in pbirch.
        const DISTANCE = 0x0800;
        /// The move is blocked by Heal Block.
        const HEAL = 0x1000;
        /// The move ignores Substitute.
        const AUTHENTIC = 0x2000;
    }
}

impl Default for Flags {
    fn default() -> Self { Flags::empty() }
}

impl FromVeekun for Flags {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
            1 ..= 14 => Flags::from_bits(1 << (value - 1)),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct FlagTable(pub HashMap<MoveId, Flags>);

impl FlagTable {
    pub fn new() -> Self {
        FlagTable::from_csv_data(vdata::MOVE_FLAGS).unwrap()
    }
}

impl vcsv::FromCsvIncremental for FlagTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: MoveId = vcsv::from_field(&record, 0)?;
        if id.0 >= 10000 {
            return Ok(())
        }
        let flag = vcsv::from_field(&record, 1)?;
        let new_flags = self.0.get(&id).map_or(flag, |v| flag | *v);
        self.0.insert(id, new_flags);
        Ok(())
    }
}

impl std::ops::Index<MoveId> for FlagTable {
    type Output = Flags;

    fn index(&self, index: MoveId) -> &Flags {
        self.0.index(&index)
    }
}
