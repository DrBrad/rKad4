use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen() -> u8 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let seed = since_the_epoch.as_nanos() as u64;

    let mut rng = Lcg::new(seed);
    rng.gen()
}

struct Lcg {
    seed: u64,
}

impl Lcg {

    fn new(seed: u64) -> Self {
        Lcg { seed }
    }

    fn gen(&mut self) -> u8 {
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        self.seed = (self.seed.wrapping_mul(A)).wrapping_add(C);
        (self.seed >> 56) as u8
    }
}
