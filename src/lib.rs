mod rng;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use crate::rng::DataraceRNG;
#[test]
fn performance_test() {
    let start = Instant::now();
    for _ in 0..1024 {
        DataraceRNG::data_race(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
    }
    println!("{:?}", start.elapsed());
}
