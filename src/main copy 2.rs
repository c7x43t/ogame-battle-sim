use once_cell::sync::Lazy;
use rand::Rng;
use rand::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

/// Compact index-based unit type IDs for efficient encoding (ships + defenses).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitType {
    // Ships
    LightFighter = 0,
    HeavyFighter = 1,
    Cruiser = 2,
    Battleship = 3,
    Recycler = 4,
    ColonyShip = 5,
    EspionageProbe = 6,
    Bomber = 7,
    SolarSatellite = 8,
    Destroyer = 9,
    DeathStar = 10,
    // Defenses
    MissileLauncher = 11,
    LightLaser = 12,
    HeavyLaser = 13,
    GaussCannon = 14,
    IonCannon = 15,
    PlasmaTurret = 16,
    SmallShieldDome = 17,
    LargeShieldDome = 18,
    AntiBallisticMissiles = 19,
    InterplanetaryMissiles = 20,
    // New Ships
    BattleCruiser = 21, // Example new ship type
    Reaper = 22,        // Example new ship type
    PathFinder = 23,    // Example new ship type
    Crawler = 24,       // Example new ship type
    SmallCargo = 25,    // Example new ship type
    LargeCargo = 26,    // Example new ship type
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

/// Static lookup table for unit stats and costs, indexable by UnitType.
static UNIT_STATS: Lazy<[UnitStats; UNIT_TYPE_COUNT]> = Lazy::new(|| {
    [
        //  0 Light Fighter
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
        //  1 Heavy Fighter
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
        //  2 Cruiser
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
        //  3 Battleship
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
        //  4 Recycler
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
        //  5 Colony Ship
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
        //  6 Espionage Probe
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
        //  7 Bomber
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
        //  8 Solar Satellite
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
        //  9 Destroyer
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
        // 10 Death Star
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
        // 11 Missile Launcher
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
        // 12 Light Laser
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
        // 13 Heavy Laser
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
        // 14 Gauss Cannon
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
        // 15 Ion Cannon
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
        // 16 Plasma Turret
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
        // 17 Small Shield Dome
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
        // 18 Large Shield Dome
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
        // 19 Anti-Ballistic Missiles
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
        // 20 Interplanetary Missiles
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
        // 21 BattleCruiser
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
        // 22 Reaper
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
        // 23 Pathfinder
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
        // 24 Crawler
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
        // 25 SmallCargo
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
        // 26 LargeCargo
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

/// Technology levels.
#[derive(Debug, Clone)]
pub struct TechLevels {
    pub weapon: u32,
    pub shield: u32,
    pub armor: u32,
}

/// Lookup rapid‑fire value.
fn rf(att: UnitType, def: UnitType) -> u8 {
    RAPID_FIRE[att as usize][def as usize]
}
/// Expected rapid‑fire multiplier: RF/(RF−1)
fn rf_mult(rf: u8) -> f64 {
    if rf <= 1 {
        1.0
    } else {
        rf as f64 / (rf as f64 - 1.0)
    }
}

/// Effective hull plating: (metal+crystal)/10 * (1+armor*0.1)
fn hull_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = &UNIT_STATS[ut as usize];
    ((stats.metal_cost + stats.crystal_cost) as f64 / 10.0) * (1.0 + 0.1 * tech.armor as f64)
}
/// Effective shield: base_shield * (1+shield*0.1)
fn shield_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = &UNIT_STATS[ut as usize];
    stats.base_shield * (1.0 + 0.1 * tech.shield as f64)
}
/// Effective attack: base_attack * (1+weapon*0.1)
fn weapon_pts(ut: UnitType, tech: &TechLevels) -> f64 {
    let stats = &UNIT_STATS[ut as usize];
    stats.base_attack * (1.0 + 0.1 * tech.weapon as f64)
}

/// Deterministic, TrashSim‑style 6‑round battle.
pub fn simulate_battle(
    atk_fleet: &Fleet,
    def_fleet: &Fleet,
    att_tech: &TechLevels,
    def_tech: &TechLevels,
    _rng: &mut ThreadRng,
) -> (Fleet, Fleet) {
    let mut atk = atk_fleet.units.clone();
    let mut def = def_fleet.units.clone();

    for _ in 0..6 {
        apply_damage_pool(&atk, att_tech, &mut def, def_tech);
        apply_damage_pool(&def, def_tech, &mut atk, att_tech);
        if atk.values().all(|&n| n == 0) || def.values().all(|&n| n == 0) {
            break;
        }
    }
    (Fleet { units: atk }, Fleet { units: def })
}

/// Distribute expected damage from attackers → targets.
fn apply_damage_pool(
    attackers: &HashMap<UnitType, u32>,
    atk_tech: &TechLevels,
    targets: &mut HashMap<UnitType, u32>,
    def_tech: &TechLevels,
) {
    if attackers.is_empty() || targets.is_empty() {
        return;
    }

    let total_targets: f64 = targets.values().copied().sum::<u32>() as f64;
    let mut total_damage: f64 = 0.0;

    // 1) compute total expected damage
    for (&ut, &count) in attackers {
        if count == 0 {
            continue;
        }
        let base_atk = weapon_pts(ut, atk_tech);

        // compute expected rapid-fire retention factor v = sum P(dut) * ((RF-1)/RF)
        let mut v: f64 = 0.0;
        for (&dut, &dcount) in targets.iter() {
            if dcount == 0 {
                continue;
            }
            let p = dcount as f64 / total_targets;
            let rf_val = rf(ut, dut);
            if rf_val > 1 {
                v += p * ((rf_val as f64 - 1.0) / rf_val as f64);
            }
        }
        // expected number of shots = 1 / (1 - v)
        let exp_mult = if v < 1.0 { 1.0 / (1.0 - v) } else { 1.0 };
        total_damage += base_atk * exp_mult * count as f64;
    }

    // 2) apply against shield then hull pools
    let mut shield_pool: f64 = targets
        .iter()
        .map(|(&ut, &n)| shield_pts(ut, def_tech) * n as f64)
        .sum();
    let mut hull_pool: f64 = targets
        .iter()
        .map(|(&ut, &n)| hull_pts(ut, def_tech) * n as f64)
        .sum();

    let mut rem = total_damage;
    // damage shields first
    let drained = rem.min(shield_pool);
    shield_pool -= drained;
    rem -= drained;
    // then hull
    let hull_dmg = rem.min(hull_pool);
    hull_pool -= hull_dmg;

    // 3) scale survivors proportionally
    let survive_ratio = if (hull_pool + hull_dmg) > 0.0 {
        hull_pool / (hull_pool + hull_dmg)
    } else {
        0.0
    };
    for (_ut, count) in targets.iter_mut() {
        *count = ((*count as f64) * survive_ratio).floor() as u32;
    }
}
fn main() {
    println!("Starting OGame battle simulation…");
    let mut attack_units = HashMap::new();
    attack_units.insert(UnitType::LightFighter, 100);
    let mut defend_units = HashMap::new();
    defend_units.insert(UnitType::MissileLauncher, 50);
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
    println!("Survivors attacker: {:?}", surv1.units);
    println!("Survivors defender: {:?}", surv2.units);
}
