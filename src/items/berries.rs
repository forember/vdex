use natures::Flavor;
use types::Type;
use vcsv;

pub const BERRY_COUNT: usize = 64;

pub struct Berry {
    pub item_id: u16,
    pub natural_gift_power: u8,
    pub natural_gift_type: Type,
    pub flavor: Option<Flavor>,
}

pub struct BerryTable {
    pub table: [Berry; BERRY_COUNT],
}

pub struct BerryFlavorTable {
    pub spicy: [u8; BERRY_COUNT],
    pub sour: [u8; BERRY_COUNT],
    pub sweet: [u8; BERRY_COUNT],
    pub dry: [u8; BERRY_COUNT],
    pub bitter: [u8; BERRY_COUNT],
}

impl vcsv::FromCsv for BerryFlavorTable {
    fn from_csv<'e, R: std::io::Read>(
        reader: &mut csv::Reader<R>
    ) -> vcsv::Result<'e, Self> {
        let mut table = BerryFlavorTable {
            spicy: [u8; BERRY_COUNT],
            sour: [u8; BERRY_COUNT],
            sweet: [u8; BERRY_COUNT],
            dry: [u8; BERRY_COUNT],
            bitter: [u8; BERRY_COUNT],
        };
        for result in reader.records() {
            let record = result?;
            let index = vcsv::from_field(&record, 0)? - 1;
            // TODO: stopping point
        }
    }
}
