use crate::types::unit_type::UnitType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Fleet {
    pub units: HashMap<UnitType, u64>,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct TechLevels {
    pub weapon: u8,
    pub shield: u8,
    pub armor: u8,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Player {
    pub fleet: Fleet,
    pub tech: TechLevels,
    // Future: Add commander boosts, alliance bonuses, etc.
}
