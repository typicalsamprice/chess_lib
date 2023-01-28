// Used in magic generation for random u64s
#[derive(Debug)]
pub struct Prng(u64);

impl Prng {
    #[inline]
    pub const fn new(val: u64) -> Self {
        debug_assert!(val > 0);
        Self(val)
    }

    fn sample(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;

        self.0.wrapping_mul(2685821657736338717)
    }

    #[inline]
    pub fn sparse(&mut self) -> u64 {
        self.sample() & self.sample() & self.sample()
    }
}