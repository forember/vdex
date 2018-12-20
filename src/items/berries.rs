use enums::Enum;
use natures::{ContestType, Flavor};
use types::Type;
use vcsv;

pub const BERRY_COUNT: usize = 64;

pub const NULL_BERRY: Berry = Berry {
    item_id: 0,
    natural_gift_power: 0,
    natural_gift_type: Type::Normal,
    flavor: None,
};

#[derive(Copy, Clone, Debug)]
pub struct Berry {
    pub item_id: u16,
    pub natural_gift_power: u8,
    pub natural_gift_type: Type,
    pub flavor: Option<Flavor>,
}

pub struct BerryTable {
    pub table: [Berry; BERRY_COUNT],
}

impl BerryTable {
    pub fn set_flavors(&mut self, flavors: &BerryFlavorTable) -> () {
        for i in 0..BERRY_COUNT {
            let mut max_flavor = None;
            let mut max_value = 0;
            for flavor in Flavor::VALUES {
                let value = flavors.flavor(*flavor)[i];
                if value > max_value {
                    max_flavor = Some(*flavor);
                    max_value = value;
                } else if value == max_value {
                    max_flavor = None;
                }
            }
            self.table[i].flavor = max_flavor;
        }
    }
}

impl vcsv::FromCsv for BerryTable {
    fn from_csv<'e, R: std::io::Read>(
        reader: &mut csv::Reader<R>
    ) -> vcsv::Result<'e, Self> {
        let mut table = BerryTable {
            table: [NULL_BERRY; BERRY_COUNT],
        };
        for result in reader.records() {
            let record = result?;
            let id: usize = vcsv::from_field(&record, 0)?;
            let item_id = vcsv::from_field(&record, 1)?;
            let natural_gift_power = vcsv::from_field(&record, 3)?;
            let natural_gift_type = vcsv::from_field(&record, 4)?;
            table.table[id - 1] = Berry {
                item_id,
                natural_gift_power,
                natural_gift_type,
                flavor: None,
            };
        }
        Ok(table)
    }
}

pub struct BerryFlavorTable {
    pub spicy: [u8; BERRY_COUNT],
    pub sour: [u8; BERRY_COUNT],
    pub sweet: [u8; BERRY_COUNT],
    pub dry: [u8; BERRY_COUNT],
    pub bitter: [u8; BERRY_COUNT],
}

impl BerryFlavorTable {
    pub fn flavor(&self, flavor: Flavor) -> &[u8; BERRY_COUNT] {
        match flavor {
            Flavor::Spicy => &self.spicy,
            Flavor::Sour => &self.sour,
            Flavor::Sweet => &self.sweet,
            Flavor::Dry => &self.dry,
            Flavor::Bitter => &self.bitter,
        }
    }

    pub fn flavor_mut(&mut self, flavor: Flavor) -> &mut [u8; BERRY_COUNT] {
        match flavor {
            Flavor::Spicy => &mut self.spicy,
            Flavor::Sour => &mut self.sour,
            Flavor::Sweet => &mut self.sweet,
            Flavor::Dry => &mut self.dry,
            Flavor::Bitter => &mut self.bitter,
        }
    }
}

impl vcsv::FromCsv for BerryFlavorTable {
    fn from_csv<'e, R: std::io::Read>(
        reader: &mut csv::Reader<R>
    ) -> vcsv::Result<'e, Self> {
        let mut table = BerryFlavorTable {
            spicy: [0; BERRY_COUNT],
            sour: [0; BERRY_COUNT],
            sweet: [0; BERRY_COUNT],
            dry: [0; BERRY_COUNT],
            bitter: [0; BERRY_COUNT],
        };
        for result in reader.records() {
            let record = result?;
            let id: usize = vcsv::from_field(&record, 0)?;
            let contest_type: ContestType = vcsv::from_field(&record, 1)?;
            let value = vcsv::from_field(&record, 2)?;
            let mut array = table.flavor_mut(Flavor::from(contest_type));
            array[id - 1] = value;
        }
        Ok(table)
    }
}
