mod simulator;
mod types;
mod utils; // This line links to simulator/mod.rs

use simulator::simulate_battle;
use simulator::simulate_battles_avg;
use std::collections::HashMap;
use std::time::Instant;
use types::player::{Fleet, Player, TechLevels};
use types::unit_type::UnitType;

fn main() {
    // Define attacker fleet
    let mut attacker_units = HashMap::new();
    attacker_units.insert(UnitType::LightFighter, 100);
    // attacker_units.insert(UnitType::Cruiser, 2);

    // Define defender fleet
    let mut defender_units = HashMap::new();
    defender_units.insert(UnitType::MissileLauncher, 50);
    // defender_units.insert(UnitType::PlasmaTurret, 1);

    // Define technology levels
    let attacker_tech = TechLevels {
        weapon: 12,
        shield: 12,
        armor: 12,
    };

    let defender_tech = TechLevels {
        weapon: 12,
        shield: 12,
        armor: 12,
    };

    // Build players
    let attacker = Player {
        fleet: Fleet {
            units: attacker_units,
        },
        tech: attacker_tech,
    };

    let defender = Player {
        fleet: Fleet {
            units: defender_units,
        },
        tech: defender_tech,
    };

    // Simulate the battle
    // simulate_battle(&attacker, &defender);
    let start = Instant::now();
    let result = simulate_battles_avg(&attacker, &defender, 100);
    let elapsed = start.elapsed();
    println!("10B FastRng::next() calls took: {:?}", elapsed);
    println!("{:?}", result);
    ////////////
    // let mut rng = FastRng::new(None);
    // println!("{}", rng.next(10_000_000_000));

    // let range_end = 10_000_000_000u64;
    // let count = 10_000_000_000u64;

    // let start = Instant::now();

    // for _ in 0..count {
    //     // Prevent optimizer from removing this
    //     black_box(rng.next(range_end));
    // }

    // let elapsed = start.elapsed();
    // println!("10B FastRng::next() calls took: {:?}", elapsed);
}
