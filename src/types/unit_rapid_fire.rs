use crate::types::unit_type::UnitType;
use once_cell::sync::Lazy;

const N: usize = UnitType::count();
/// Rapid-fire lookup table: shots on average per firing sequence.
const RAPID_FIRE_RULES: &[(UnitType, UnitType, u16)] = &[
    // SmallCargo vs Espionage Probes and Solar Satellites
    (UnitType::SmallCargo, UnitType::EspionageProbe, 5),
    (UnitType::SmallCargo, UnitType::SolarSatellite, 5),
    (UnitType::SmallCargo, UnitType::Crawler, 5),
    // LargeCargo vs Espionage Probes and Solar Satellites
    (UnitType::LargeCargo, UnitType::EspionageProbe, 5),
    (UnitType::LargeCargo, UnitType::SolarSatellite, 5),
    (UnitType::LargeCargo, UnitType::Crawler, 5),
    // Light Fighter vs Espionage Probes and Solar Satellites
    (UnitType::LightFighter, UnitType::EspionageProbe, 5),
    (UnitType::LightFighter, UnitType::SolarSatellite, 5),
    (UnitType::LightFighter, UnitType::Crawler, 5),
    // Heavy Fighter vs Espionage Probes, Solar Satellites, Light Fighters
    (UnitType::HeavyFighter, UnitType::EspionageProbe, 5),
    (UnitType::HeavyFighter, UnitType::SolarSatellite, 5),
    (UnitType::HeavyFighter, UnitType::Crawler, 5),
    (UnitType::HeavyFighter, UnitType::SmallCargo, 3),
    // Cruiser vs Espionage Probes, Solar Satellites, Light Fighters, Missile Launchers
    (UnitType::Cruiser, UnitType::EspionageProbe, 5),
    (UnitType::Cruiser, UnitType::SolarSatellite, 5),
    (UnitType::Cruiser, UnitType::Crawler, 5),
    (UnitType::Cruiser, UnitType::LightFighter, 6),
    (UnitType::Cruiser, UnitType::MissileLauncher, 10),
    // Battleship vs Espionage Probes and Solar Satellites
    (UnitType::Battleship, UnitType::EspionageProbe, 5),
    (UnitType::Battleship, UnitType::SolarSatellite, 5),
    (UnitType::Battleship, UnitType::Crawler, 5),
    (UnitType::Battleship, UnitType::PathFinder, 5),
    (UnitType::BattleCruiser, UnitType::EspionageProbe, 5),
    (UnitType::BattleCruiser, UnitType::SolarSatellite, 5),
    (UnitType::BattleCruiser, UnitType::Crawler, 5),
    (UnitType::BattleCruiser, UnitType::HeavyFighter, 4),
    (UnitType::BattleCruiser, UnitType::Cruiser, 4),
    (UnitType::BattleCruiser, UnitType::Battleship, 7),
    (UnitType::BattleCruiser, UnitType::SmallCargo, 3),
    (UnitType::BattleCruiser, UnitType::LargeCargo, 3),
    // Recycler vs Espionage Probes and Solar Satellites
    (UnitType::Recycler, UnitType::EspionageProbe, 5),
    (UnitType::Recycler, UnitType::SolarSatellite, 5),
    (UnitType::Recycler, UnitType::Crawler, 5),
    // Colony Ship vs Espionage Probes and Solar Satellites
    (UnitType::ColonyShip, UnitType::EspionageProbe, 5),
    (UnitType::ColonyShip, UnitType::SolarSatellite, 5),
    (UnitType::ColonyShip, UnitType::Crawler, 5),
    // Bomber vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters, Cruisers, Battleships
    (UnitType::Bomber, UnitType::EspionageProbe, 5),
    (UnitType::Bomber, UnitType::SolarSatellite, 5),
    (UnitType::Bomber, UnitType::Crawler, 5),
    (UnitType::Bomber, UnitType::MissileLauncher, 20),
    (UnitType::Bomber, UnitType::LightLaser, 20),
    (UnitType::Bomber, UnitType::HeavyLaser, 10),
    (UnitType::Bomber, UnitType::IonCannon, 10),
    (UnitType::Bomber, UnitType::GaussCannon, 5),
    (UnitType::Bomber, UnitType::PlasmaTurret, 5),
    // Destroyer vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters
    (UnitType::Destroyer, UnitType::EspionageProbe, 5),
    (UnitType::Destroyer, UnitType::SolarSatellite, 5),
    (UnitType::Destroyer, UnitType::Crawler, 5),
    (UnitType::Destroyer, UnitType::LightLaser, 10),
    (UnitType::Destroyer, UnitType::BattleCruiser, 2),
    // Death Star vs Espionage Probes, Solar Satellites, Light Fighters, Heavy Fighters,
    // Cruisers, Battleships, Colony Ships, Recyclers, Bombers, Destroyers, Death Stars
    (UnitType::DeathStar, UnitType::EspionageProbe, 250),
    (UnitType::DeathStar, UnitType::SolarSatellite, 250),
    (UnitType::DeathStar, UnitType::LightFighter, 200),
    (UnitType::DeathStar, UnitType::HeavyFighter, 100),
    (UnitType::DeathStar, UnitType::Cruiser, 33),
    (UnitType::DeathStar, UnitType::Battleship, 30),
    (UnitType::DeathStar, UnitType::Bomber, 25),
    (UnitType::DeathStar, UnitType::Destroyer, 5),
    (UnitType::DeathStar, UnitType::SmallCargo, 250),
    (UnitType::DeathStar, UnitType::LargeCargo, 250),
    (UnitType::DeathStar, UnitType::ColonyShip, 250),
    (UnitType::DeathStar, UnitType::Recycler, 250),
    (UnitType::DeathStar, UnitType::LightLaser, 200),
    (UnitType::DeathStar, UnitType::HeavyLaser, 100),
    (UnitType::DeathStar, UnitType::IonCannon, 100),
    (UnitType::DeathStar, UnitType::GaussCannon, 50),
    (UnitType::DeathStar, UnitType::BattleCruiser, 15),
    (UnitType::DeathStar, UnitType::PathFinder, 30),
    (UnitType::DeathStar, UnitType::Reaper, 10),
    (UnitType::DeathStar, UnitType::Crawler, 250),
    (UnitType::Reaper, UnitType::EspionageProbe, 5),
    (UnitType::Reaper, UnitType::SolarSatellite, 5),
    (UnitType::Reaper, UnitType::Crawler, 5),
    (UnitType::Reaper, UnitType::Battleship, 7),
    (UnitType::Reaper, UnitType::Bomber, 4),
    (UnitType::Reaper, UnitType::Destroyer, 3),
    (UnitType::PathFinder, UnitType::EspionageProbe, 5),
    (UnitType::PathFinder, UnitType::SolarSatellite, 5),
    (UnitType::PathFinder, UnitType::Crawler, 5),
    (UnitType::PathFinder, UnitType::Cruiser, 3),
    (UnitType::PathFinder, UnitType::LightFighter, 3),
    (UnitType::PathFinder, UnitType::HeavyFighter, 2),
];
pub static RAPID_FIRE: Lazy<[[u16; N]; N]> = Lazy::new(|| {
    // start with every entry = 1
    let mut m = [[1u16; N]; N];

    // apply each sparse rule
    for &(att, def, shots) in RAPID_FIRE_RULES {
        m[att as usize][def as usize] = shots;
    }

    m
});

pub fn rapid_fire_for(attacker: UnitType, defender: UnitType) -> u16 {
    RAPID_FIRE[attacker as usize][defender as usize]
}
