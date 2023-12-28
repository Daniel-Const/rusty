/**
 * Lib.rs
 *
 * Some helper functions
 */

use rand::{thread_rng, Rng};

/**
 * Generate a vector of n ints in range min..max
 */
pub fn generate_vec(n: u32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec = (0..n).map(|_| rng.gen_range(min..max)).collect::<Vec<_>>();
    vec.sort();
    return vec;
}
