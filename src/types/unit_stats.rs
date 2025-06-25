use crate::types::unit_type::UnitType;
use once_cell::sync::Lazy;
use std::ops::Index;

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
#[allow(unused)]
pub static UNIT_STATS: Lazy<[UnitStats; UnitType::count()]> = Lazy::new(|| {
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
            base_shield: 25.0,
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
            base_shield: 100.0,
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
            base_shield: 500.0,
            hull: 8000.0,
            metal_cost: 2000.0,
            crystal_cost: 6000.0,
            deuterium_cost: 0.0,
            speed: 0.0,
            cargo: 0.0,
        },
        // 22 Plasma Turret
        UnitStats {
            base_attack: 3000.0,
            base_shield: 300.0,
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
            base_shield: 2000.0,
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
            base_shield: 10000.0,
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
impl Index<UnitType> for [UnitStats; UnitType::count()] {
    type Output = UnitStats;
    fn index(&self, unit: UnitType) -> &Self::Output {
        &self[unit as usize]
    }
}
#[allow(unused)]
/// Convenience accessor
pub fn stats_for(unit: UnitType) -> &'static UnitStats {
    &UNIT_STATS[unit]
}
