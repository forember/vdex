use FromVeekun;
use std::collections::HashMap;
use vcsv;

bitflags! {
    /// Miscellaneous bitflags for items.
    pub struct Flags: u8 {
        /// The item can stack in the bag.
        const Countable = 0x01;
        /// The item is consumed when used.
        const Consumable = 0x02;
        /// The item is usable out of battle.
        const UsableOverworld = 0x04;
        /// The item is usable in battle.
        const UsableInBattle = 0x08;
        /// The item can be held by a Pokémon.
        const Holdable = 0x10;
        /// When held by a Pokémon, the effect applies without active use.
        const HoldablePassive = 0x20;
        /// When held by a Pokémon, the effect requires active use.
        const HoldableActive = 0x40;
        /// The item can appear in the Sinnoh Underground.
        const Underground = 0x80;
    }
}

impl FromVeekun for Flags {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
            1 ... 8 => Flags::from_bits(1 << (value - 1)),
            _ => None,
        }
    }
}

pub struct FlagTable(pub HashMap<u16, Flags>);

impl vcsv::FromCsvIncremental for FlagTable {
    fn from_empty_csv() -> Self {
        FlagTable(HashMap::new())
    }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id = vcsv::from_field(&record, 0)?;
        let flag = vcsv::from_field(&record, 1)?;
        let new_flags = self.0.get(&id).map_or(flag, |v| flag | *v);
        self.0.insert(id, new_flags);
        Ok(())
    }
}
