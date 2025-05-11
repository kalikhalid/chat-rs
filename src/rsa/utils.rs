use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::{rngs::OsRng, Fill, RngCore, thread_rng};


pub fn rabbin_test(n: &BigUint, k: u32) -> bool {
    if n <= &BigUint::one() || n.is_even() {
        return false;
    }
    if n == &3u8.to_biguint().unwrap() {
        return true;
    }

    // разложение n - 1 = 2^r * s
    let one = BigUint::one();
    let two = &one + &one;
    let mut s = n - &one;
    let mut r = 0u32;
    while s.is_even() {
        s >>= 1; // быстрее, чем s /= 2
        r += 1;
    }

    let mut rng = thread_rng();
    'witness_loop: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &(n - &one)); // a ∈ [2, n-2]
        let mut x = a.modpow(&s, n);

        if x == one || x == n - &one {
            continue;
        }

        for _ in 0..(r - 1) {
            x = x.modpow(&two, n);
            if x == n - &one {
                continue 'witness_loop;
            }
        }
        return false; // составное число
    }
    true // вероятно простое
}

pub fn gen_prime() -> BigUint {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut candidate;
    
    loop {
        candidate = rng.gen_biguint(2048); 
        candidate.set_bit(0, true); 
        
        if !is_divisible_by_small_primes(&candidate) {
            continue;
        }
        
        if rabbin_test(&candidate, 5) {
            return candidate;
        }
    }
}

fn is_divisible_by_small_primes(n: &BigUint) -> bool {
    let small_primes = [3u32, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for &p in &small_primes {
        if n % p == BigUint::zero() {
            return false;
        }
    }
    true
}
