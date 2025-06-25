use crate::types::player::TechLevels;
use crate::types::unit_stats::UNIT_STATS;
use crate::types::unit_type::UnitType;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Unit {
    pub hull: f64,
    pub shield: f64,
    pub unit_type: UnitType,
}

impl Unit {
    /// Creates a unit with computed hull and shield points
    #[allow(unused)]
    pub fn new(unit_type: UnitType, tech: &TechLevels) -> Self {
        Self {
            hull: Self::hull_pts(unit_type, tech),
            shield: Self::shield_pts(unit_type, tech),
            unit_type,
        }
    }

    /// Effective hull = base_hull * (1 + 0.1 * armor_tech) / 10.0
    #[allow(unused)]
    pub fn hull_pts(ut: UnitType, tech: &TechLevels) -> f64 {
        let stats = UNIT_STATS[ut as usize];
        stats.hull * (1.0 + 0.1 * tech.armor as f64) / 10.0
    }

    /// Effective shield = base_shield * (1 + 0.1 * shield_tech)
    #[allow(unused)]
    pub fn shield_pts(ut: UnitType, tech: &TechLevels) -> f64 {
        let stats = UNIT_STATS[ut as usize];
        stats.base_shield * (1.0 + 0.1 * tech.shield as f64)
    }

    /// Effective attack = base_attack * (1 + 0.1 * weapon_tech)
    #[allow(unused)]
    pub fn attack_pts(unit_type: UnitType, tech: &TechLevels) -> f64 {
        let stats = UNIT_STATS[unit_type as usize];
        stats.base_attack * (1.0 + 0.1 * tech.weapon as f64)
    }
}
