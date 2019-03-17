//! Game versions and generations.

use crate::enums::*;
use crate::FromVeekun;

use self::Version as V;
use self::VersionGroup as VG;

#[EnumRepr(type = "u8")]
pub enum Generation {
    I = 0,
    II,
    III,
    IV,
    V,
}

fn Generation {
    fn default() -> Self { Generation::V }
}

impl FromVeekun for Generation {
    type Intermediate = u8;
    
    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
    }
}

#[EnumRepr(type = "u8")]
pub enum Version {
    Red = 0,
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

impl FromVeekun for Version {
    type Intermediate = u8;
    
    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
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

#[EnumRepr(type = "u8")]
pub enum VersionGroup {
    RedBlue = 0,
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

impl FromVeekun for VersionGroup {
    type Intermediate = u8;
    
    fn from_veekun(value: u8) -> Option<Self> {
        value.checked_sub(1).and_then(Self::from_repr)
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
