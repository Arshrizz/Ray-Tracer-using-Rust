use crate::vec3::Vec3;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Returns a random point in the unit disk using the provided RNG.
pub fn random_in_unit_disk_rng(rng: &mut StdRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Creates a deterministic per-pixel RNG seeded from pixel coordinates and a global seed.
pub fn make_rng(x: u32, y: u32, seed: u64) -> StdRng {
    // Combine x, y, and seed into a single u64 seed
    let mut seed_val = seed;
    seed_val = seed_val.wrapping_add(x as u64 * 374761393);
    seed_val = seed_val.wrapping_add(y as u64 * 668265263);
    seed_val = seed_val.wrapping_mul(1664525);
    seed_val = seed_val.wrapping_add(1013904223);
    StdRng::seed_from_u64(seed_val)
}
