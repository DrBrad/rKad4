use std::time::{SystemTime, UNIX_EPOCH};

/*
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
*/
pub fn gen<T>() -> T where T: From<u8> {
    let mut rng = Lcg::new(get_seed());
    rng.gen::<T>()
}

pub fn gen_array<const N: usize>() -> [u8; N] {
    let mut rng = Lcg::new(get_seed());
    let mut array = [0u8; N];
    for i in 0..N {
        array[i] = rng.gen();
    }
    array
}

fn get_seed() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_nanos() as u64
}

struct Lcg {
    seed: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { seed }
    }

    fn gen<T>(&mut self) -> T where T: From<u8> {
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        self.seed = (self.seed.wrapping_mul(A)).wrapping_add(C);
        T::from((self.seed >> 56) as u8)
    }
}
