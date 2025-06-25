use rand::{RngCore, SeedableRng, TryRngCore, rngs::OsRng};
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct FastRng {
    rng: Xoshiro256PlusPlus,
}

impl FastRng {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(|| OsRng.try_next_u64().unwrap());
        Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn next(&mut self, end: u64) -> u64 {
        debug_assert!(end > 0);
        self.rng.next_u64() % end
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn next_inclusive(&mut self, end: u64) -> u64 {
        debug_assert!(end > 0 && end < u64::MAX);
        self.rng.next_u64() % (end + 1)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn next_f64(&mut self) -> f64 {
        // take the top 53 bits of a 64-bit random value
        let bits = self.rng.next_u64() >> 11; // down to 53 bits
        // divide by 2^53 as f64
        bits as f64 / (1u64 << 53) as f64
    }

    /// Uniform in [0.0, 1.0)
    #[inline(always)]
    #[allow(dead_code)]
    pub fn next_f32(&mut self) -> f32 {
        // take the top 24 bits of a 64-bit random value
        let bits = (self.rng.next_u64() >> 40) as u32; // down to 24 bits
        bits as f32 / (1u32 << 24) as f32
    }
}
