use FromVeekun;
use std::collections::HashMap;
use vcsv;

bitflags! {
    pub struct Flags: u8 {
        const Countable = 0x01;
        const Consumable = 0x02;
        const UsableOverworld = 0x04;
        const UsableInBattle = 0x08;
        const Holdable = 0x10;
        const HoldablePassive = 0x20;
        const HoldableActive = 0x40;
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
        self.0.insert(id, flag);
        Ok(())
    }
}
