use crate::types::player::Player;
use crate::types::rapid_fire_for;
use crate::types::unit::Unit;
use crate::types::unit_type::{UnitType, units_map_to_array};
use crate::utils::fast_rng::FastRng;
use std::iter;

pub fn simulate_battle(
    attacker: &Player,
    defender: &Player,
) -> ([u64; UnitType::count()], [u64; UnitType::count()]) {
    // println!("Simulating battle...");
    let mut rng = FastRng::new(None);
    let attacker_fleet = &attacker.fleet;
    let defender_fleet = &defender.fleet;
    let attacker_units = &attacker_fleet.units;
    let defender_units = &defender_fleet.units;

    let mut n_attacker_units = attacker_units.values().copied().sum::<u64>();
    let mut n_defender_units = defender_units.values().copied().sum::<u64>();

    // 2) Pre‐allocate the vectors
    let mut attackers = Vec::with_capacity(n_attacker_units as usize);
    let mut defenders = Vec::with_capacity(n_defender_units as usize);
    // println!("{:?}", attackers);
    // 3) Fill by cloning a small prototype for each type
    //    This does ONE Unit::new per unit‐type, then repeats it `count` times cheaply.
    for (&ut, &count) in attacker_units {
        let proto = Unit::new(ut, &attacker.tech);
        attackers.extend(iter::repeat(proto).take(count as usize));
    }
    for (&ut, &count) in defender_units {
        let proto = Unit::new(ut, &defender.tech);
        defenders.extend(iter::repeat(proto).take(count as usize));
    }
    let mut attacker_units_tracker = units_map_to_array(&attacker_units);
    let mut defender_units_tracker = units_map_to_array(&defender_units);

    // println!("Battle result: {:?}", attacker_units_tracker);

    for _round in 0..6 {
        // iterate attacker units tracker
        let attacker_units_tracker_clone = attacker_units_tracker.clone();
        let defender_units_tracker_clone = defender_units_tracker.clone();
        let mut tmp_n_defender_units = n_defender_units;
        let mut tmp_n_attacker_units = n_attacker_units;
        // ─── Attacker units attack ───────────────────────────────────────
        for (unit_type_index, &count) in attacker_units_tracker_clone.iter().enumerate() {
            let attacker_unit_type: UnitType = UnitType::ALL[unit_type_index];
            let dmg = Unit::attack_pts(attacker_unit_type, &attacker.tech);
            for _ in 0..count {
                let mut keep_firing = true; // rapid fire loop
                while keep_firing && tmp_n_defender_units > 0 {
                    // Find a defender unit that can be hit
                    let defender_index = {
                        // rng.next takes a u64, so cast the bound up
                        let bound = tmp_n_defender_units as u64;
                        rng.next(bound) as usize
                    };

                    let defender_unit = &mut defenders[defender_index];
                    let defender_max_shield =
                        Unit::shield_pts(defender_unit.unit_type, &defender.tech);
                    if defender_unit.shield > 0.0 {
                        // 1 % “bounce” rule (≤ 1 % of full shield does nothing)
                        let pct = dmg / defender_max_shield * 100.0;
                        if pct <= 1.0 {
                            break; // skip this hit;
                        }
                        let overflow_damage = f64::max(0.0, dmg - defender_unit.shield);
                        defender_unit.shield = f64::max(0.0, defender_unit.shield - dmg);
                        defender_unit.hull = defender_unit.hull - overflow_damage;
                    } else {
                        defender_unit.hull = defender_unit.hull - dmg;
                    }
                    if defender_unit.hull <= 0.0 {
                        // kill the defender unit
                        defender_units_tracker[defender_unit.unit_type as usize] -= 1;
                        defenders
                            .swap(defender_index as usize, (tmp_n_defender_units - 1) as usize);
                        tmp_n_defender_units -= 1;
                        break;
                    }
                    let max_hull = Unit::hull_pts(defender_unit.unit_type, &defender.tech);
                    if defender_unit.hull < 0.7 * max_hull {
                        let explode_chance = 1.0 - (defender_unit.hull / max_hull);
                        if rng.next_f64() < explode_chance {
                            // kill the defender unit
                            defender_units_tracker[defender_unit.unit_type as usize] -= 1;
                            defenders
                                .swap(defender_index as usize, (tmp_n_defender_units - 1) as usize);
                            tmp_n_defender_units -= 1;
                            break;
                        }
                    }

                    let rf = rapid_fire_for(attacker_unit_type, defender_unit.unit_type);
                    keep_firing = rf > 1 && rng.next_f32() < (rf as f32 - 1.0) / rf as f32;
                }
            }
        }

        // ─── Defender units attack ───────────────────────────────────────
        for (unit_type_index, &count) in defender_units_tracker_clone.iter().enumerate() {
            let defender_unit_type = UnitType::ALL[unit_type_index];
            let dmg = Unit::attack_pts(defender_unit_type, &defender.tech);

            for _ in 0..count {
                let mut keep_firing = true;
                while keep_firing && tmp_n_attacker_units > 0 {
                    // pick a random live attacker
                    let bound = tmp_n_attacker_units as u64;
                    let atk_idx = rng.next(bound) as usize;

                    // exactly like the attacker code: mutably borrow once…
                    let au = &mut attackers[atk_idx];
                    let max_shield = Unit::shield_pts(au.unit_type, &attacker.tech);
                    if au.shield > 0.0 {
                        let pct = dmg / max_shield * 100.0;
                        if pct <= 1.0 {
                            break; // bounced
                        }
                        let overflow = f64::max(0.0, dmg - au.shield);
                        au.shield = f64::max(0.0, au.shield - dmg);
                        au.hull -= overflow;
                    } else {
                        au.hull -= dmg;
                    }

                    // if it died, update tracker + swap it out of the live region
                    if au.hull <= 0.0 {
                        attacker_units_tracker[au.unit_type as usize] -= 1;
                        attackers.swap(atk_idx, (tmp_n_attacker_units - 1) as usize);
                        tmp_n_attacker_units -= 1;
                        // println!("Attacker unit destroyed");
                        break;
                    }

                    // explosion‐kill check
                    let max_hull = Unit::hull_pts(au.unit_type, &attacker.tech);
                    if au.hull < 0.7 * max_hull {
                        let explode_chance = 1.0 - (au.hull / max_hull);
                        if rng.next_f64() < explode_chance {
                            attacker_units_tracker[au.unit_type as usize] -= 1;
                            attackers.swap(atk_idx, (tmp_n_attacker_units - 1) as usize);
                            tmp_n_attacker_units -= 1;
                            // println!("Attacker unit destroyed");
                            break;
                        }
                    }

                    // rapid‐fire loop condition
                    let rf = rapid_fire_for(defender_unit_type, au.unit_type);
                    keep_firing = rf > 1 && rng.next_f32() < (rf as f32 - 1.0) / rf as f32;
                }
            }
        }
        // ────────────────────────────────────────────────────────────────────
        // reset shields
        for i in 0..(n_defender_units as usize) {
            let ut = defenders[i].unit_type;
            defenders[i].shield = Unit::shield_pts(ut, &defender.tech);
        }
        for i in 0..(n_attacker_units as usize) {
            let ut = attackers[i].unit_type;
            attackers[i].shield = Unit::shield_pts(ut, &attacker.tech);
        }
        n_defender_units = tmp_n_defender_units;
        n_attacker_units = tmp_n_attacker_units;
    }

    return (attacker_units_tracker, defender_units_tracker);
}

pub fn simulate_battles_avg(
    attacker: &Player,
    defender: &Player,
    trials: usize,
) -> ([u64; UnitType::count()], [u64; UnitType::count()]) {
    // Accumulators in u64, using saturating_add to avoid panics on overflow
    let mut sum_att = [0u64; UnitType::count()];
    let mut sum_def = [0u64; UnitType::count()];

    // Run the simulations
    for _ in 0..trials {
        let (att_rem, def_rem) = simulate_battle(attacker, defender);
        for i in 0..UnitType::count() {
            sum_att[i] = sum_att[i].saturating_add(att_rem[i]);
            sum_def[i] = sum_def[i].saturating_add(def_rem[i]);
        }
    }

    // Compute rounded averages
    let mut avg_att = [0u64; UnitType::count()];
    let mut avg_def = [0u64; UnitType::count()];
    let inv = 1.0 / (trials as f64);
    for i in 0..UnitType::count() {
        avg_att[i] = (sum_att[i] as f64 * inv).round() as u64;
        avg_def[i] = (sum_def[i] as f64 * inv).round() as u64;
    }

    (avg_att, avg_def)
}
