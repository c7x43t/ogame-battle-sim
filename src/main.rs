use fastrand;
use once_cell::sync::Lazy;
use rand::Rng;
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Index;
thread_local! {
    static GLOBAL_RNG: RefCell<fastrand::Rng> = RefCell::new(fastrand::Rng::new());
}
//////
/// Compact index-based unit type IDs for efficient encoding (ships + defenses).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    // Ships (in firing order)
    SmallCargo = 0,
    LargeCargo = 1,
    LightFighter = 2,
    HeavyFighter = 3,
    Cruiser = 4,
    Battleship = 5,
    ColonyShip = 6,
    Recycler = 7,
    EspionageProbe = 8,
    Bomber = 9,
    SolarSatellite = 10,
    Destroyer = 11,
    DeathStar = 12,
    BattleCruiser = 13,
    Reaper = 14,
    PathFinder = 15,
    Crawler = 16, // Doesn’t fire but included

    // Defenses (in firing order)
    MissileLauncher = 17, // Rocket Launcher
    LightLaser = 18,
    HeavyLaser = 19,
    GaussCannon = 20,
    IonCannon = 21,
    PlasmaTurret = 22,
    SmallShieldDome = 23,
    LargeShieldDome = 24,

    // Special (not in firing sequence)
    AntiBallisticMissiles = 25,
    InterplanetaryMissiles = 26,
}
impl TryFrom<u8> for UnitType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UnitType::SmallCargo),
            1 => Ok(UnitType::LargeCargo),
            2 => Ok(UnitType::LightFighter),
            3 => Ok(UnitType::HeavyFighter),
            4 => Ok(UnitType::Cruiser),
            5 => Ok(UnitType::Battleship),
            6 => Ok(UnitType::ColonyShip),
            7 => Ok(UnitType::Recycler),
            8 => Ok(UnitType::EspionageProbe),
            9 => Ok(UnitType::Bomber),
            10 => Ok(UnitType::SolarSatellite),
            11 => Ok(UnitType::Destroyer),
            12 => Ok(UnitType::DeathStar),
            13 => Ok(UnitType::BattleCruiser),
            14 => Ok(UnitType::Reaper),
            15 => Ok(UnitType::PathFinder),
            16 => Ok(UnitType::Crawler),
            17 => Ok(UnitType::MissileLauncher),
            18 => Ok(UnitType::LightLaser),
            19 => Ok(UnitType::HeavyLaser),
            20 => Ok(UnitType::GaussCannon),
            21 => Ok(UnitType::IonCannon),
            22 => Ok(UnitType::PlasmaTurret),
            23 => Ok(UnitType::SmallShieldDome),
            24 => Ok(UnitType::LargeShieldDome),
            25 => Ok(UnitType::AntiBallisticMissiles),
            26 => Ok(UnitType::InterplanetaryMissiles),
            _ => Err(()),
        }
    }
}

/// Total number of unit types defined
const UNIT_TYPE_COUNT: usize = 27;

/// Unit statistics container, including deuterium cost.
#[derive(Debug, Clone, Copy)]
pub struct UnitStats {
    pub base_attack: f64,
    pub base_shield: f64,
    pub hull: f64, // NEW FIELD
    pub metal_cost: f64,
    pub crystal_cost: f64,
    pub deuterium_cost: f64,
    pub speed: f64, // in units/hour
    pub cargo: f64, // in resource units
}

static UNIT_STATS: Lazy<[UnitStats; UNIT_TYPE_COUNT]> = Lazy::new(|| {
    [
        // 0 SmallCargo
        UnitStats {
            base_attack: 1.0,
            base_shield: 1.0,
            hull: 4000.0,
            metal_cost: 2000.0,
            crystal_cost: 2000.0,
            deuterium_cost: 0.0,
            speed: 5000.0,
            cargo: 5000.0,
        },
        // 1 LargeCargo
        UnitStats {
            base_attack: 1.0,
            base_shield: 1.0,
            hull: 12000.0,
            metal_cost: 6000.0,
            crystal_cost: 6000.0,
            deuterium_cost: 0.0,
            speed: 7500.0,
            cargo: 25000.0,
        },
        // 2 Light Fighter
        UnitStats {
            base_attack: 50.0,
            base_shield: 10.0,
            hull: 4000.0,
            metal_cost: 3000.0,
            crystal_cost: 1000.0,
            deuterium_cost: 0.0,
            speed: 12000.0,
            cargo: 50.0,
        },
        // 3 Heavy Fighter
        UnitStats {
            base_attack: 150.0,
            base_shield: 25.0,
            hull: 10000.0,
            metal_cost: 6000.0,
            crystal_cost: 4000.0,
            deuterium_cost: 0.0,
            speed: 10000.0,
            cargo: 100.0,
        },
        // 4 Cruiser
        UnitStats {
            base_attack: 400.0,
            base_shield: 50.0,
            hull: 27000.0,
            metal_cost: 20000.0,
            crystal_cost: 7000.0,
            deuterium_cost: 2000.0,
            speed: 15000.0,
            cargo: 800.0,
        },
        // 5 Battleship
        UnitStats {
            base_attack: 1000.0,
            base_shield: 200.0,
            hull: 60000.0,
            metal_cost: 45000.0,
            crystal_cost: 15000.0,
            deuterium_cost: 0.0,
            speed: 10000.0,
            cargo: 1500.0,
        },
        // 6 Colony Ship
        UnitStats {
            base_attack: 1.0,
            base_shield: 10.0,
            hull: 30000.0,
            metal_cost: 10000.0,
            crystal_cost: 20000.0,
            deuterium_cost: 10000.0,
            speed: 2500.0,
            cargo: 5000.0,
        },
        // 7 Recycler
        UnitStats {
            base_attack: 1.0,
            base_shield: 10.0,
            hull: 16000.0,
            metal_cost: 10000.0,
            crystal_cost: 6000.0,
            deuterium_cost: 0.0,
            speed: 2000.0,
            cargo: 2000.0,
        },
        // 8 Espionage Probe
        UnitStats {
            base_attack: 0.0,
            base_shield: 0.0,
            hull: 1000.0,
            metal_cost: 0.0,
            crystal_cost: 1000.0,
            deuterium_cost: 0.0,
            speed: 20000.0,
            cargo: 5.0,
        },
        // 9 Bomber
        UnitStats {
            base_attack: 1000.0,
            base_shield: 100.0,
            hull: 75000.0,
            metal_cost: 50000.0,
            crystal_cost: 25000.0,
            deuterium_cost: 15000.0,
            speed: 4000.0,
            cargo: 500.0,
        },
        // 10 Solar Satellite
        UnitStats {
            base_attack: 1.0,
            base_shield: 20.0,
            hull: 2000.0,
            metal_cost: 0.0,
            crystal_cost: 2000.0,
            deuterium_cost: 500.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 11 Destroyer
        UnitStats {
            base_attack: 2000.0,
            base_shield: 500.0,
            hull: 110000.0,
            metal_cost: 60000.0,
            crystal_cost: 50000.0,
            deuterium_cost: 15000.0,
            speed: 10000.0,
            cargo: 2000.0,
        },
        // 12 Death Star
        UnitStats {
            base_attack: 90000.0,
            base_shield: 50000.0,
            hull: 9000000.0,
            metal_cost: 5000000.0,
            crystal_cost: 4000000.0,
            deuterium_cost: 1000000.0,
            speed: 1000.0,
            cargo: 100000.0,
        },
        // 13 BattleCruiser
        UnitStats {
            base_attack: 700.0,
            base_shield: 400.0,
            hull: 70000.0,
            metal_cost: 30000.0,
            crystal_cost: 40000.0,
            deuterium_cost: 15000.0,
            speed: 10000.0,
            cargo: 750.0,
        },
        // 14 Reaper
        UnitStats {
            base_attack: 2800.0,
            base_shield: 700.0,
            hull: 140000.0,
            metal_cost: 85000.0,
            crystal_cost: 55000.0,
            deuterium_cost: 20000.0,
            speed: 7000.0,
            cargo: 10000.0,
        },
        // 15 Pathfinder
        UnitStats {
            base_attack: 200.0,
            base_shield: 100.0,
            hull: 23000.0,
            metal_cost: 8000.0,
            crystal_cost: 15000.0,
            deuterium_cost: 8000.0,
            speed: 12000.0,
            cargo: 10000.0,
        },
        // 16 Crawler
        UnitStats {
            base_attack: 1.0,
            base_shield: 1.0,
            hull: 4000.0,
            metal_cost: 2000.0,
            crystal_cost: 3000.0,
            deuterium_cost: 1000.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 17 Missile Launcher
        UnitStats {
            base_attack: 80.0,
            base_shield: 20.0,
            hull: 2000.0,
            metal_cost: 2000.0,
            crystal_cost: 0.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 18 Light Laser
        UnitStats {
            base_attack: 100.0,
            base_shield: 10.0,
            hull: 2000.0,
            metal_cost: 1500.0,
            crystal_cost: 500.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 19 Heavy Laser
        UnitStats {
            base_attack: 250.0,
            base_shield: 25.0,
            hull: 8000.0,
            metal_cost: 6000.0,
            crystal_cost: 2000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 20 Gauss Cannon
        UnitStats {
            base_attack: 1100.0,
            base_shield: 200.0,
            hull: 35000.0,
            metal_cost: 20000.0,
            crystal_cost: 15000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 21 Ion Cannon
        UnitStats {
            base_attack: 150.0,
            base_shield: 300.0,
            hull: 8000.0,
            metal_cost: 2000.0,
            crystal_cost: 6000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 22 Plasma Turret
        UnitStats {
            base_attack: 4000.0,
            base_shield: 1000.0,
            hull: 100000.0,
            metal_cost: 50000.0,
            crystal_cost: 50000.0,
            deuterium_cost: 30000.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 23 Small Shield Dome
        UnitStats {
            base_attack: 1.0,
            base_shield: 1000.0,
            hull: 20000.0,
            metal_cost: 10000.0,
            crystal_cost: 10000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 24 Large Shield Dome
        UnitStats {
            base_attack: 1.0,
            base_shield: 5000.0,
            hull: 100000.0,
            metal_cost: 50000.0,
            crystal_cost: 50000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 25 Anti-Ballistic Missiles
        UnitStats {
            base_attack: 1.0,
            base_shield: 1.0,
            hull: 8000.0,
            metal_cost: 8000.0,
            crystal_cost: 0.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 26 Interplanetary Missiles
        UnitStats {
            base_attack: 1.0,
            base_shield: 1.0,
            hull: 12500.0,
            metal_cost: 12500.0,
            crystal_cost: 0.0,
            deuterium_cost: 0.0,
            speed: 30000.0,
            cargo: 0.0,
        },
    ]
});

/// Allow indexing UNIT_STATS by UnitType directly.
impl Index<UnitType> for [UnitStats; UNIT_TYPE_COUNT] {
    type Output = UnitStats;
    fn index(&self, unit: UnitType) -> &Self::Output {
        &self[unit as usize]
    }
}

/// Rapid-fire lookup table: shots on average per firing sequence.
static RAPID_FIRE: Lazy<[[u8; UNIT_TYPE_COUNT]; UNIT_TYPE_COUNT]> = Lazy::new(|| {
    let mut m = [[1u8; UNIT_TYPE_COUNT]; UNIT_TYPE_COUNT];

    // SmallCargo vs Espionage Probes and Solar Satellites
    m[UnitType::SmallCargo as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::SmallCargo as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::SmallCargo as usize][UnitType::Crawler as usize] = 5;

    // LargeCargo vs Espionage Probes and Solar Satellites
    m[UnitType::LargeCargo as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::LargeCargo as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::LargeCargo as usize][UnitType::Crawler as usize] = 5;

    // Light Fighter vs Espionage Probes and Solar Satellites
    m[UnitType::LightFighter as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::LightFighter as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::LightFighter as usize][UnitType::Crawler as usize] = 5;

    // Heavy Fighter vs Espionage Probes, Solar Satellites, Light Fighters
    m[UnitType::HeavyFighter as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::HeavyFighter as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::HeavyFighter as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::HeavyFighter as usize][UnitType::SmallCargo as usize] = 3;

    // Cruiser vs Espionage Probes, Solar Satellites, Light Fighters, Missile Launchers
    m[UnitType::Cruiser as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Cruiser as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Cruiser as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::Cruiser as usize][UnitType::LightFighter as usize] = 6;
    m[UnitType::Cruiser as usize][UnitType::MissileLauncher as usize] = 10;

    // Battleship vs Espionage Probes and Solar Satellites
    m[UnitType::Battleship as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Battleship as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Battleship as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::Battleship as usize][UnitType::PathFinder as usize] = 5;

    m[UnitType::BattleCruiser as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::BattleCruiser as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::BattleCruiser as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::BattleCruiser as usize][UnitType::HeavyFighter as usize] = 4;
    m[UnitType::BattleCruiser as usize][UnitType::Cruiser as usize] = 4;
    m[UnitType::BattleCruiser as usize][UnitType::Battleship as usize] = 7;
    m[UnitType::BattleCruiser as usize][UnitType::SmallCargo as usize] = 3;
    m[UnitType::BattleCruiser as usize][UnitType::LargeCargo as usize] = 3;

    // Recycler vs Espionage Probes and Solar Satellites
    m[UnitType::Recycler as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Recycler as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Recycler as usize][UnitType::Crawler as usize] = 5;

    // Colony Ship vs Espionage Probes and Solar Satellites
    m[UnitType::ColonyShip as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::ColonyShip as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::ColonyShip as usize][UnitType::Crawler as usize] = 5;

    // Bomber vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters, Cruisers, Battleships
    m[UnitType::Bomber as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Bomber as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Bomber as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::Bomber as usize][UnitType::MissileLauncher as usize] = 20;
    m[UnitType::Bomber as usize][UnitType::LightLaser as usize] = 20;
    m[UnitType::Bomber as usize][UnitType::HeavyLaser as usize] = 10;
    m[UnitType::Bomber as usize][UnitType::IonCannon as usize] = 10;
    m[UnitType::Bomber as usize][UnitType::GaussCannon as usize] = 5;
    m[UnitType::Bomber as usize][UnitType::PlasmaTurret as usize] = 5;

    // Destroyer vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters
    m[UnitType::Destroyer as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Destroyer as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Destroyer as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::Destroyer as usize][UnitType::LightLaser as usize] = 10;
    m[UnitType::Destroyer as usize][UnitType::BattleCruiser as usize] = 2;

    // Death Star vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters,
    // Cruisers, Battleships, Colony Ships, Recyclers, Bombers, Destroyers, Death Stars
    m[UnitType::DeathStar as usize][UnitType::EspionageProbe as usize] = 250;
    m[UnitType::DeathStar as usize][UnitType::SolarSatellite as usize] = 250;
    m[UnitType::DeathStar as usize][UnitType::LightFighter as usize] = 200;
    m[UnitType::DeathStar as usize][UnitType::HeavyFighter as usize] = 100;
    m[UnitType::DeathStar as usize][UnitType::Cruiser as usize] = 33;
    m[UnitType::DeathStar as usize][UnitType::Battleship as usize] = 30;

    m[UnitType::DeathStar as usize][UnitType::Bomber as usize] = 25;
    m[UnitType::DeathStar as usize][UnitType::Destroyer as usize] = 5;
    m[UnitType::DeathStar as usize][UnitType::SmallCargo as usize] = 250;
    m[UnitType::DeathStar as usize][UnitType::LargeCargo as usize] = 250;
    m[UnitType::DeathStar as usize][UnitType::ColonyShip as usize] = 250;
    m[UnitType::DeathStar as usize][UnitType::Recycler as usize] = 250;

    m[UnitType::DeathStar as usize][UnitType::LightLaser as usize] = 200;
    m[UnitType::DeathStar as usize][UnitType::HeavyLaser as usize] = 100;
    m[UnitType::DeathStar as usize][UnitType::IonCannon as usize] = 100;
    m[UnitType::DeathStar as usize][UnitType::GaussCannon as usize] = 50;
    m[UnitType::DeathStar as usize][UnitType::BattleCruiser as usize] = 15;
    m[UnitType::DeathStar as usize][UnitType::PathFinder as usize] = 30;
    m[UnitType::DeathStar as usize][UnitType::Reaper as usize] = 10;
    m[UnitType::DeathStar as usize][UnitType::Crawler as usize] = 250;

    m[UnitType::Reaper as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::Reaper as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::Reaper as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::Reaper as usize][UnitType::Battleship as usize] = 7;
    m[UnitType::Reaper as usize][UnitType::Bomber as usize] = 4;
    m[UnitType::Reaper as usize][UnitType::Destroyer as usize] = 3;

    m[UnitType::PathFinder as usize][UnitType::EspionageProbe as usize] = 5;
    m[UnitType::PathFinder as usize][UnitType::SolarSatellite as usize] = 5;
    m[UnitType::PathFinder as usize][UnitType::Crawler as usize] = 5;
    m[UnitType::PathFinder as usize][UnitType::Cruiser as usize] = 3;
    m[UnitType::PathFinder as usize][UnitType::LightFighter as usize] = 3;
    m[UnitType::PathFinder as usize][UnitType::HeavyFighter as usize] = 2;

    m
});
/// Access rapid-fire value from table.

/// Fleet composition mapping.
#[derive(Debug, Clone)]
pub struct Fleet {
    pub units: HashMap<UnitType, u32>,
}
impl Fleet {
    pub fn from_counts(units: HashMap<UnitType, u32>) -> Self {
        Fleet { units }
    }

    /// convenience: build a FastRandomPool directly from this Fleet
    pub fn to_pool(&self, tech: &TechLevels) -> FastRandomPool {
        fleet_to_pool(self, tech)
    }
    pub fn into_counts(self) -> HashMap<UnitType, u32> {
        self.units
    }
}

/// Technology levels.
#[derive(Debug, Clone)]
pub struct TechLevels {
    pub weapon: u32,
    pub shield: u32,
    pub armor: u32,
}
/// Effective hull plating: (metal+crystal)/10 * (1+armor*0.1)
fn hull_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = UNIT_STATS[ut as usize];
    stats.hull * (1.0 + 0.1 * tech.armor as f64) / 10.0
}

/// Effective shield: base_shield * (1 + shield tech * 0.1)
fn shield_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = UNIT_STATS[ut as usize];
    stats.base_shield * (1.0 + 0.1 * tech.shield as f64)
}

/// Effective attack: base_attack * (1 + weapon tech * 0.1)
fn weapon_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = UNIT_STATS[ut as usize];
    stats.base_attack * (1.0 + 0.1 * tech.weapon as f64)
}

/////// Data Structure
#[derive(Debug)]
pub struct FastRandomPool {
    pub hulls: Vec<f32>,
    pub shields: Vec<f32>,
    pub unit_types: Vec<UnitType>,
    pub alive: Vec<bool>,

    alive_idx: Vec<usize>, // dense list of alive entity indices
    pos_of: Vec<usize>,    // entity-index → position in `alive_idx`

    tech: TechLevels,
}

impl FastRandomPool {
    /// Build from a `Fleet` + tech, so we can
    /// both init current HP/shield *and* know how to recompute max later.
    pub fn new(fleet: &Fleet, tech: TechLevels) -> Self {
        let total = fleet.units.values().map(|&c| c as usize).sum();
        let mut pool = FastRandomPool {
            hulls: Vec::with_capacity(total),
            shields: Vec::with_capacity(total),
            unit_types: Vec::with_capacity(total),
            alive: Vec::with_capacity(total),
            alive_idx: Vec::with_capacity(total),
            pos_of: Vec::with_capacity(total),
            tech,
        };

        let mut idx = 0;
        for (&ut, &count) in &fleet.units {
            // compute once
            let full_h = hull_pts(ut, &pool.tech) as f32;
            let full_s = shield_pts(ut, &pool.tech) as f32;

            for _ in 0..count {
                pool.unit_types.push(ut);
                pool.hulls.push(full_h);
                pool.shields.push(full_s);
                pool.alive.push(true);
                pool.alive_idx.push(idx);
                pool.pos_of.push(idx);
                idx += 1;
            }
        }
        pool
    }
    pub fn into_fleet(&self) -> Fleet {
        pool_to_fleet(self)
    }
    #[inline(always)]
    pub fn random_alive(&self) -> Option<usize> {
        if self.alive_idx.is_empty() {
            None
        } else {
            let i = fastrand::usize(..self.alive_idx.len());
            Some(self.alive_idx[i])
        }
    }

    #[inline(always)]
    fn kill(&mut self, idx: usize) {
        self.alive[idx] = false;
        let pos = self.pos_of[idx];
        let last = *self.alive_idx.last().unwrap();
        self.alive_idx.swap_remove(pos);
        if pos < self.alive_idx.len() {
            self.pos_of[last] = pos;
        }
    }

    #[inline(always)]
    pub fn alive_count(&self) -> usize {
        self.alive_idx.len()
    }

    pub fn apply_shot(&mut self, idx: usize, dmg: f32) {
        // ------------- constants for this target -------------
        let max_shield = shield_pts(self.unit_types[idx], &self.tech) as f32; // full shield
        let full_sh = max_shield;
        let shield = &mut self.shields[idx];
        let hull = &mut self.hulls[idx];

        // ------------- branch A — shield still up -------------
        if *shield >= 0.0 {
            // 1 % “bounce” rule (≤ 1 % of full shield does nothing)
            let pct = dmg / full_sh * 100.0;
            if pct <= 1.0 {
                return;
            }

            // A1: attack smaller than full shield  → percentage erosion
            if dmg < full_sh {
                let dmg_pct = pct.floor(); // whole-% chunks
                let delta = dmg_pct * 0.01 * full_sh; // absolute shield loss
                let mut new_sh = *shield - delta;

                // replicate TrashSim’s fractional remainder rule
                if new_sh == 0.0 && (pct - dmg_pct) > 0.0 {
                    new_sh -= (pct - dmg_pct) * 0.01 * full_sh;
                }
                *shield = new_sh;
                // no hull damage in this branch
                return;
            }

            // A2: attack ≥ full shield → shield collapses
            let leftover = if *shield > 0.0 { dmg - *shield } else { dmg };
            *shield = -1.0; // sentinel “destroyed”
            *hull -= leftover;
        }
        // ------------- branch B — shield already down -------------
        else {
            *hull -= dmg;
        }

        // ------------- death & explosion check -------------
        if *hull <= 0.0 {
            return self.kill(idx);
        }
        let max_hull = hull_pts(self.unit_types[idx], &self.tech) as f32; // combat HP
        if *hull < 0.7 * max_hull {
            let explode_chance = 1.0 - (*hull / max_hull);
            if fastrand::f32() < explode_chance {
                self.kill(idx);
            }
        }
    }

    pub fn reset_shields(&mut self) {
        for (i, &ut) in self.unit_types.iter().enumerate() {
            if self.alive[i] {
                self.shields[i] = shield_pts(ut, &self.tech) as f32;
            }
        }
    }
}
////

/// Main simulate_battle with deterministic expected-value logic.
///
/// Converts a Fleet into a FastRandomPool using tech levels.
/// Each unit's hull, shield, and UnitType is initialized based on the fleet composition.
fn fleet_to_pool(fleet: &Fleet, tech: &TechLevels) -> FastRandomPool {
    let total_units: usize = fleet.units.values().map(|&v| v as usize).sum();
    let mut pool = FastRandomPool {
        hulls: Vec::with_capacity(total_units),
        shields: Vec::with_capacity(total_units),
        unit_types: Vec::with_capacity(total_units),
        alive: Vec::with_capacity(total_units),
        alive_idx: Vec::with_capacity(total_units),
        pos_of: Vec::with_capacity(total_units),
        tech: tech.clone(),
    };

    let mut idx = 0;
    for (&unit_type, &count) in &fleet.units {
        let max_hull = hull_pts(unit_type, tech) as f32;
        let max_shield = shield_pts(unit_type, tech) as f32;

        for _ in 0..count {
            pool.unit_types.push(unit_type);
            pool.hulls.push(max_hull);
            pool.shields.push(max_shield);
            pool.alive.push(true);
            pool.alive_idx.push(idx);
            pool.pos_of.push(idx);
            idx += 1;
        }
    }

    pool
}
fn pool_alive_counts(pool: &FastRandomPool) -> HashMap<UnitType, u32> {
    let mut counts = HashMap::new();
    for &idx in &pool.alive_idx {
        let ut = pool.unit_types[idx];
        *counts.entry(ut).or_insert(0) += 1;
    }
    counts
}

fn pool_to_fleet(pool: &FastRandomPool) -> Fleet {
    let mut unit_counts: HashMap<UnitType, u32> = HashMap::new();

    for &idx in &pool.alive_idx {
        let unit_type = pool.unit_types[idx];
        *unit_counts.entry(unit_type).or_insert(0) += 1;
    }

    Fleet { units: unit_counts }
}
pub fn simulate_battle(
    attacker: &Fleet,
    defender: &Fleet,
    att_tech: &TechLevels,
    def_tech: &TechLevels,
    _rng: &mut ThreadRng,
) -> (Fleet, Fleet) {
    let mut atk_units = attacker.units.clone();
    let mut def_units = defender.units.clone();
    let mut attacker_pool = fleet_to_pool(attacker, att_tech);
    let mut defender_pool = fleet_to_pool(defender, def_tech);

    for _round in 0..6 {
        // Take snapshots of the fleets at the start of the round (alive at round-start)
        // let atk_snapshot = atk_units.clone();
        // let def_snapshot = def_units.clone();
        let atk_snapshot = pool_alive_counts(&attacker_pool);
        let def_snapshot = pool_alive_counts(&defender_pool);
        // ——————————————
        // Attackers → Defenders
        // ——————————————
        let ordered_attackers = (0u8..=16u8)
            .filter_map(|v| {
                UnitType::try_from(v).ok().and_then(|ut| {
                    atk_snapshot
                        .get(&ut)
                        .filter(|&&cnt| cnt > 0)
                        .map(|&cnt| (ut, cnt))
                })
            })
            .collect::<Vec<_>>();

        for (att_ut, count) in ordered_attackers {
            let base_dmg = weapon_pts(att_ut, att_tech) as f32;
            for _ in 0..count {
                let mut keep_firing = true;
                while keep_firing {
                    if let Some(def_idx) = defender_pool.random_alive() {
                        defender_pool.apply_shot(def_idx, base_dmg);

                        let def_ut = defender_pool.unit_types[def_idx];
                        let rf = RAPID_FIRE[att_ut as usize][def_ut as usize].max(1);
                        keep_firing = rf > 1 && fastrand::f32() < (rf as f32 - 1.0) / rf as f32;
                    } else {
                        keep_firing = false;
                    }
                }
            }
        }

        // ——————————————
        // Defenders → Attackers
        // ——————————————
        let ordered_defenders = (0u8..=16u8)
            .filter_map(|v| {
                UnitType::try_from(v).ok().and_then(|ut| {
                    def_snapshot
                        .get(&ut)
                        .filter(|&&cnt| cnt > 0)
                        .map(|&cnt| (ut, cnt))
                })
            })
            .collect::<Vec<_>>();

        for (def_ut, count) in ordered_defenders {
            let base_dmg = weapon_pts(def_ut, def_tech) as f32;
            for _ in 0..count {
                let mut keep_firing = true;
                while keep_firing {
                    if let Some(atk_idx) = attacker_pool.random_alive() {
                        attacker_pool.apply_shot(atk_idx, base_dmg);

                        let atk_ut = attacker_pool.unit_types[atk_idx];
                        let rf = RAPID_FIRE[def_ut as usize][atk_ut as usize].max(1);
                        keep_firing = rf > 1 && fastrand::f32() < (rf as f32 - 1.0) / rf as f32;
                    } else {
                        keep_firing = false;
                    }
                }
            }
        }

        // ——————————————
        // End-of-round updates
        // ——————————————
        atk_units = pool_to_fleet(&attacker_pool).units;
        def_units = pool_to_fleet(&defender_pool).units;

        if attacker_pool.alive_count() == 0 || defender_pool.alive_count() == 0 {
            break;
        }

        attacker_pool.reset_shields();
        defender_pool.reset_shields();
    }

    (Fleet { units: atk_units }, Fleet { units: def_units })
}

fn main() {
    println!("Starting OGame battle simulation…");
    let mut attack_units = HashMap::new();
    attack_units.insert(UnitType::LightFighter, 1);
    let mut defend_units = HashMap::new();
    defend_units.insert(UnitType::MissileLauncher, 1);
    let fleet1 = Fleet {
        units: attack_units,
    };
    let fleet2 = Fleet {
        units: defend_units,
    };
    let tech = TechLevels {
        weapon: 12,
        shield: 12,
        armor: 12,
    };
    let mut rng = rand::rng();
    let (surv1, surv2) = simulate_battle(&fleet1, &fleet2, &tech, &tech, &mut rng);
    println!("Survivors attacker: {:?}", surv1);
    println!("Survivors defender: {:?}", surv2);
}
