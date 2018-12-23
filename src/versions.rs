//! Game versions and generations.

use enums::*;
use FromVeekun;

#[EnumRepr(type = "u8")]
pub enum Generation {
    I = 1,
    II,
    III,
    IV,
    V,
}

#[EnumRepr(type = "u8")]
pub enum Version {
    Red = 1,
    Blue,
    Yellow,
    Gold,
    Silver,
    Crystal,
    Ruby,
    Sapphire,
    Emerald,
    Firered,
    Leafgreen,
    Diamond,
    Pearl,
    Platinum,
    Heartgold,
    Soulsilver,
    Black,
    White,
    Colosseum,
    XD,
    Black2,
    White2,
}

#[EnumRepr(type = "u8")]
pub enum VersionGroup {
    RedBlue = 1,
    Yellow,
    GoldSilver,
    Crystal,
    RubySapphire,
    Emerald,
    FireredLeafgreen,
    DiamondPearl,
    Platinum,
    HeartgoldSoulsilver,
    BlackWhite,
    Colosseum,
    XD,
    BlackWhite2,
}

use versions::Version as V;
use versions::VersionGroup as VG;

impl FromVeekun for Generation {
    type Intermediate = u8;
    
    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

impl FromVeekun for Version {
    type Intermediate = u8;
    
    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

impl FromVeekun for VersionGroup {
    type Intermediate = u8;
    
    fn from_veekun(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }
}

impl Version {
    pub fn group(self) -> VersionGroup {
        match self {
            V::Red | V::Blue => VG::RedBlue,
            V::Yellow => VG::Yellow,
            V::Gold | V::Silver => VG::GoldSilver,
            V::Crystal => VG::Crystal,
            V::Ruby | V::Sapphire => VG::RubySapphire,
            V::Emerald => VG::Emerald,
            V::Firered | V::Leafgreen => VG::FireredLeafgreen,
            V::Diamond | V::Pearl => VG::DiamondPearl,
            V::Platinum => VG::Platinum,
            V::Heartgold | V::Soulsilver => VG::HeartgoldSoulsilver,
            V::Black | V::White => VG::BlackWhite,
            V::Colosseum => VG::Colosseum,
            V::XD => VG::XD,
            V::Black2 | V::White2 => VG::BlackWhite2,
        }
    }

    pub fn generation(self) -> Generation {
        self.group().generation()
    }
}

impl VersionGroup {
    pub fn generation(self) -> Generation {
        match self {
            VG::RedBlue | VG::Yellow => Generation::I,
            VG::GoldSilver | VG::Crystal => Generation::II,
            VG::RubySapphire | VG::Emerald | VG::FireredLeafgreen
                | VG::Colosseum | VG::XD => Generation::III,
            VG::DiamondPearl | VG::Platinum | VG::HeartgoldSoulsilver
                => Generation::IV,
            VG::BlackWhite | VG::BlackWhite2 => Generation::V,
        }
    }
}
