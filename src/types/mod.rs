// 1) Declare the two submodules:
pub mod player;
pub mod unit;
pub mod unit_rapid_fire;
pub mod unit_stats;
pub mod unit_type;
#[allow(unused)]
pub use unit_rapid_fire::{RAPID_FIRE, rapid_fire_for};
#[allow(unused)]
pub use unit_stats::{UNIT_STATS, UnitStats, stats_for};
