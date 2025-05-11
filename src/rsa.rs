use num_bigint::{BigUint, RandBigInt};
use rand::RngCore;

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
    let (mut r, mut s): (BigUint, BigUint) = (BigUint::ZERO, n-1u8);
    while &s % 2u8 == BigUint::ZERO{
        r += 1u8;
        s /= 2u8;
    }
    for _ in 0..k{
        let a = RngCore::fill_bytes();
    }

    
    false
}
