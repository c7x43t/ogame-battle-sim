// src/types/unit_type.rs

use core::slice;
use std::collections::HashMap;

const N: usize = 27;
/// All the unit types in OGame
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    SmallCargo,
    LargeCargo,
    LightFighter,
    HeavyFighter,
    Cruiser,
    Battleship,
    ColonyShip,
    Recycler,
    EspionageProbe,
    Bomber,
    SolarSatellite,
    Destroyer,
    DeathStar,
    BattleCruiser,
    Reaper,
    PathFinder,
    Crawler,

    MissileLauncher,
    LightLaser,
    HeavyLaser,
    GaussCannon,
    IonCannon,
    PlasmaTurret,
    SmallShieldDome,
    LargeShieldDome,

    AntiBallisticMissiles,
    InterplanetaryMissiles,
}

impl UnitType {
    /// Single source-of-truth array of all variants
    pub const ALL: [UnitType; N] = [
        UnitType::SmallCargo,
        UnitType::LargeCargo,
        UnitType::LightFighter,
        UnitType::HeavyFighter,
        UnitType::Cruiser,
        UnitType::Battleship,
        UnitType::ColonyShip,
        UnitType::Recycler,
        UnitType::EspionageProbe,
        UnitType::Bomber,
        UnitType::SolarSatellite,
        UnitType::Destroyer,
        UnitType::DeathStar,
        UnitType::BattleCruiser,
        UnitType::Reaper,
        UnitType::PathFinder,
        UnitType::Crawler,
        UnitType::MissileLauncher,
        UnitType::LightLaser,
        UnitType::HeavyLaser,
        UnitType::GaussCannon,
        UnitType::IonCannon,
        UnitType::PlasmaTurret,
        UnitType::SmallShieldDome,
        UnitType::LargeShieldDome,
        UnitType::AntiBallisticMissiles,
        UnitType::InterplanetaryMissiles,
    ];

    /// Number of variants (kept in sync)
    pub const fn count() -> usize {
        Self::ALL.len()
    }

    /// Iterate all variants
    pub fn iter() -> slice::Iter<'static, UnitType> {
        Self::ALL.iter()
    }

    /// Try to convert a `u8` into a `UnitType`
    pub fn from_u8(n: u8) -> Option<Self> {
        // Safe because repr(u8) and ALL is ordered
        let idx = n as usize;
        if idx < Self::ALL.len() {
            Some(Self::ALL[idx])
        } else {
            None
        }
    }
}

// Allow easy casts back to u8, usize, etc.
impl From<UnitType> for u8 {
    fn from(u: UnitType) -> u8 {
        u as u8
    }
}
impl From<UnitType> for usize {
    fn from(u: UnitType) -> usize {
        u as usize
    }
}

impl From<i32> for UnitType {
    fn from(value: i32) -> Self {
        let idx = value as usize;
        if idx < Self::ALL.len() {
            // SAFETY: idx in bounds of ALL
            Self::ALL[idx]
        } else {
            panic!("Invalid UnitType index: {}", value);
        }
    }
}

pub fn units_map_to_array(map: &HashMap<UnitType, u64>) -> [u64; N] {
    let mut arr = [0u64; N];
    for (&ut, &cnt) in map {
        arr[ut as usize] = cnt;
    }
    arr
}
