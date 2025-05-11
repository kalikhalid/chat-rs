use tokio::io::{stdin, stdout};
use crate::rsa::rsa::KeyPair;
use num_bigint::BigUint;

mod rsa;

#[tokio::main]
async fn main() {
    let kp = KeyPair::new(None, None);
    println!("{:?}", kp.print());
}
