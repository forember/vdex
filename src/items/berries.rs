use crate::enums::*;
use crate::FromVeekun;
use super::ItemId;
use crate::Type;
use crate::vcsv;
use crate::vcsv::FromCsv;
use crate::vdata;

/// Aka condition, the "type" of moves in contests.
///
/// The only use of condition in pbirch is the association with berry flavors,
/// as contests are out of the scope of pbirch.
#[EnumRepr(type = "u8")]
pub enum ContestType {
    Cool = 0,
    Tough,
    Cute,
    Beauty,
    Smart,
}

/// Berry flavor, which Pokémon may like or dislike.
///
/// Used in conjunction with nature to determine the effect of Figy, Wiki, Mago,
/// Aguav, and Iapapa berries.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Flavor)
/// > Flavor is a special set of attributes that certain foods in the Pokémon
/// > world have.  The affected foods include Berries, Pokéblocks, Poffins,
/// > Apricorns, Aprijuice, and malasadas. There are five different flavors:
/// >
/// > - Spicy (Japanese: 辛い spicy)
/// > - Dry (Japanese: 渋い astringent)
/// > - Sweet (Japanese: 甘い sweet)
/// > - Bitter (Japanese: 苦い bitter)
/// > - Sour (Japanese: 酸っぱい sour)
/// >
/// > Most of the foods can have more than one flavor attributed to them.
/// > 
/// > The primary flavor mechanic is taste preferences: different Pokémon like
/// > and dislike different flavors, depending on their Nature. This usually
/// > influences the effectiveness of the food. The flavors that a Pokémon likes
/// > and dislikes correspond to the stat that its Nature raises or lowers. For
/// > example, all Pokémon who like spicy flavors have a Nature that raises
/// > their Attack, while those that dislike spicy flavors have a Nature that
/// > lowers it.  Pokémon who have neutral Natures have no likes or dislikes.
#[EnumRepr(type = "u8")]
pub enum Flavor {
    Spicy = 0,
    Sour,
    Sweet,
    Dry,
    Bitter,
}

impl std::convert::From<Flavor> for ContestType {
    fn from(flavor: Flavor) -> Self {
        ContestType::from_repr(flavor.repr()).unwrap()
    }
}

impl FromVeekun for ContestType {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        match value {
            1 => Some(ContestType::Cool),
            2 => Some(ContestType::Beauty),
            3 => Some(ContestType::Cute),
            4 => Some(ContestType::Smart),
            5 => Some(ContestType::Tough),
            _ => None,
        }
    }
}

impl std::convert::From<ContestType> for Flavor {
    fn from(contest: ContestType) -> Self {
        Flavor::from_repr(contest.repr()).unwrap()
    }
}

impl FromVeekun for Flavor {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        ContestType::from_veekun(value).and_then(|t| Some(Flavor::from(t)))
    }
}

/// The total number of berries in pbirch.
pub const BERRY_COUNT: usize = 64;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct BerryId(pub u8);

impl Default for BerryId {
    fn default() -> Self { BerryId(std::u8::MAX) }
}

impl FromVeekun for BerryId {
    type Intermediate = u8;

    fn from_veekun(value: u8) -> Option<Self> {
        let id = value.checked_sub(1).unwrap_or_default();
        if id < (BERRY_COUNT as u8) {
            Some(BerryId(id))
        } else {
            None
        }
    }
}

/// A held item that a Pokémon can use in battle.
///
/// > [*[From Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Berry)
/// > Berries (Japanese: きのみ Tree Fruit) are small, juicy, fleshy fruit. As
/// > in the real world, a large variety exists in the Pokémon world, with a
/// > large range of flavors, names, and effects. First found in the Generation
/// > II games, many Berries have since become critical held items in battle,
/// > where their various effects include HP and status condition restoration,
/// > stat enhancement, and even damage negation.
#[derive(Copy, Clone, Debug, Default)]
pub struct Berry {
    pub item: ItemId,
    pub natural_gift_power: u8,
    pub natural_gift_type: Type,
    pub flavor: Option<Flavor>,
}

pub struct BerryTable(pub [Berry; BERRY_COUNT]);

impl BerryTable {
    pub fn new() -> Self {
        let mut table = BerryTable::from_csv_data(vdata::BERRIES).unwrap();
        table.set_flavors(&BerryFlavorTable::new());
        table
    }

    fn set_flavors(&mut self, flavors: &BerryFlavorTable) {
        for id in 0..BERRY_COUNT {
            let mut max_flavor = None;
            let mut max_value = 0;
            for &flavor in Flavor::VALUES {
                let value = flavors[flavor][id];
                if value > max_value {
                    max_flavor = Some(flavor);
                    max_value = value;
                } else if value == max_value {
                    max_flavor = None;
                }
            }
            self.0[id].flavor = max_flavor;
        }
    }
}

impl Default for BerryTable {
    fn default() -> Self {
        BerryTable([Default::default(); BERRY_COUNT])
    }
}

impl vcsv::FromCsvIncremental for BerryTable {
    fn from_empty_csv() -> Self { Default::default() }
    
    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: BerryId = vcsv::from_field(&record, 0)?;
        self[id] = Berry {
            item: vcsv::from_field(&record, 1)?,
            natural_gift_power: vcsv::from_field(&record, 3)?,
            natural_gift_type: vcsv::from_field(&record, 4)?,
            flavor: None,
        };
        Ok(())
    }
}

impl std::ops::Index<BerryId> for BerryTable {
    type Output = Berry;

    fn index(&self, index: BerryId) -> &Berry {
        self.0.index(index.0 as usize)
    }
}

impl std::ops::IndexMut<BerryId> for BerryTable {
    fn index_mut(&mut self, index: BerryId) -> &mut Berry {
        self.0.index_mut(index.0 as usize)
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
    pub fn new() -> Self {
        BerryFlavorTable::from_csv_data(vdata::BERRY_FLAVORS).unwrap()
    }
}

impl Default for BerryFlavorTable {
    fn default() -> Self {
        BerryFlavorTable {
            spicy: [0; BERRY_COUNT],
            sour: [0; BERRY_COUNT],
            sweet: [0; BERRY_COUNT],
            dry: [0; BERRY_COUNT],
            bitter: [0; BERRY_COUNT],
        }
    }
}

impl vcsv::FromCsvIncremental for BerryFlavorTable {
    fn from_empty_csv() -> Self { Default::default() }

    fn load_csv_record(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<()> {
        let id: BerryId = vcsv::from_field(&record, 0)?;
        let contest_type: ContestType = vcsv::from_field(&record, 1)?;
        let flavor = Flavor::from(contest_type);
        let value = vcsv::from_field(&record, 2)?;
        self[flavor][id.0 as usize] = value;
        Ok(())
    }
}

impl std::ops::Index<Flavor> for BerryFlavorTable {
    type Output = [u8; BERRY_COUNT];

    fn index<'a>(&'a self, index: Flavor) -> &'a [u8; BERRY_COUNT] {
        match index {
            Flavor::Spicy => &self.spicy,
            Flavor::Sour => &self.sour,
            Flavor::Sweet => &self.sweet,
            Flavor::Dry => &self.dry,
            Flavor::Bitter => &self.bitter,
        }
    }
}

impl std::ops::IndexMut<Flavor> for BerryFlavorTable {
    fn index_mut<'a>(&'a mut self, index: Flavor) -> &'a mut [u8; BERRY_COUNT] {
        match index {
            Flavor::Spicy => &mut self.spicy,
            Flavor::Sour => &mut self.sour,
            Flavor::Sweet => &mut self.sweet,
            Flavor::Dry => &mut self.dry,
            Flavor::Bitter => &mut self.bitter,
        }
    }
}
