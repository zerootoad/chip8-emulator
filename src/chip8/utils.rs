use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::SystemTime;

pub fn rand_byte() -> u8 {
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let mut rng = StdRng::seed_from_u64(d.as_secs());
    rng.gen_range(0..255)
}
