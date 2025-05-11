use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;

fn rabbin_test(n: &BigUint, k: u32) -> bool{
    if n % 2u8 == BigUint::ZERO{
        return false;
    }
    if *n <= BigUint::from(1u8){
        return false;
    }
    if *n <= BigUint::from(3u8){
        return true;
    }
    let mut rng = thread_rng();
    let (mut r, mut s): (BigUint, BigUint) = (BigUint::ZERO, n-1u8);
    while &s % 2u8 == BigUint::ZERO{
        r += 1u8;
        s /= 2u8;
    }
    for _ in 0..k{
        let a = rng.gen_biguint_range(&BigUint::from(2u8), n);
        let x = a.modpow(&s, n);
        if x == 1u8 || x == n - 1u8{
            continue;
        }
        for __ in 0..(r-1){
            
        }
    }

    
    false
}
