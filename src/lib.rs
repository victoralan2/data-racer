use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::thread::{sleep, spawn};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::{Rng, RngCore, SeedableRng};
use rand::distributions::{Distribution, Standard};
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::rngs::StdRng;

pub struct DataraceRNG {
    state: u128,
}
impl Default for DataraceRNG {
    fn default() -> Self {
        let start = SystemTime::now();
        let seed = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_nanos();
        let state = Self::basic_hash(seed);
        Self { state }
    }
}
impl DataraceRNG {
    pub fn new(seed: u128) -> Self {
        let state = Self::basic_hash(seed);
        Self { state }
    }
    fn basic_hash(seed: u128) -> u128 {
        let mut seed = seed;
        seed ^= seed.wrapping_shr(26);
        seed ^= seed.wrapping_shl(13);
        seed ^= seed.wrapping_shr(23);
        seed
    }
    fn mutate(&mut self) {
        self.state = Self::data_race(self.state);
    }
    pub fn data_race(state: u128) -> u128{
        let mut state = state;
        let state = Arc::new(AtomicPtr::new(&mut state));
        unsafe {
            let mut thread_pool = Vec::new();
            for i in 0..12u32 {
                let state = state.clone();
                let t = spawn(move || {
                    for e in 0..1024 {
                        let state_value = *state.load(Ordering::Relaxed);
                        let new_value = Self::basic_hash(!1945678154678958719829601872597819u128.wrapping_mul(i.wrapping_shr(e) as u128).wrapping_shl(e.wrapping_pow(i)) as u128 ^ state_value);
                        let mut newer_value = u128::from_le_bytes(new_value.to_be_bytes());
                        state.store(&mut newer_value, Ordering::Release);
                    }
                });
                thread_pool.push(t);
            }
            for thread in thread_pool{
                thread.join().unwrap();
            }
            sleep(Duration::new(0, 100000000));
            *state.load(Ordering::Relaxed)
        }
    }

    fn blake3(data: u128) -> u128 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&data.to_be_bytes());
        let result = hasher.finalize();
        let bytes: [u8; 16] = slice_to_fixed_array(result.as_bytes());
        u128::from_le_bytes(bytes)
    }
    fn blake3_raw(data: u128) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&data.to_be_bytes());
        let result = hasher.finalize();
        *result.as_bytes()
    }
    fn result(&self) -> u128{
        Self::blake3(self.state)
    }
    pub fn gen<T>(&mut self) -> T
        where Standard: Distribution<T>{
        let mut rng = StdRng::from_seed(Self::blake3_raw(self.result()));
        self.mutate();
        rng.gen()
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
        where T: SampleUniform, R: SampleRange<T>,{
        let mut rng = StdRng::from_seed(Self::blake3_raw(self.result()));
        self.mutate();
        rng.gen_range(range)
    }
}
impl RngCore for DataraceRNG {
    fn next_u32(&mut self) -> u32 {
        self.gen()
    }

    fn next_u64(&mut self) -> u64 {
        self.gen()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut rng = StdRng::from_seed(Self::blake3_raw(self.result()));
        self.mutate();
        rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        let mut rng = StdRng::from_seed(Self::blake3_raw(self.result()));
        self.mutate();
        rng.fill_bytes(dest);
        Ok(())
    }
}
fn slice_to_fixed_array(source_slice: &[u8]) -> [u8; 16] {
    let mut target_array: [u8; 16] = [0; 16];

    // If the source_slice is longer than the target_array, truncate it.
    let slice_len = source_slice.len().min(16);
    target_array[..slice_len].copy_from_slice(&source_slice[..slice_len]);

    target_array
}
