use num_bigint::BigUint;
use crate::rsa::utils::{rabbin_test, gen_prime};

const RSA_EXP: u32 = 65537u32;

struct PublicKey{
    pub e: BigUint, 
    pub n: BigUint,
}

struct PrivateKey{
    p: BigUint, 
    q: BigUint, 
    d: BigUint, 
    phi_n: BigUint,
}

pub struct KeyPair{
    pub public: PublicKey,
    pub private: PrivateKey,
}

impl KeyPair{
    pub fn new(p: Option<BigUint>, q: Option<BigUint>) -> Self{
        let p = if let Some(p) = p { p } else { gen_prime() };
        let q = if let Some(q) = q { q } else { gen_prime() };
        let e = BigUint::from(RSA_EXP);
        let n = &p * &q;
        let phi_n = (&p - 1u8)*(&q-1u8);
        let d = e.modinv(&phi_n).unwrap();
        let public = PublicKey { e, n };
        let private = PrivateKey { p, q, d, phi_n };

        Self { public, private } 
    }
    pub fn print(&self) -> (BigUint, BigUint,BigUint,BigUint){
        (self.private.p.clone(), self.private.q.clone(), self.private.d.clone(), self.private.phi_n.clone()) 
    }
}







