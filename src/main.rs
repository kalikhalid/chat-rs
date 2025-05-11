use tokio::io::{stdin, stdout};
use rsa::rabbin_test;
use num_bigint::BigUint;

mod rsa;

#[tokio::main]
async fn main() {
    println!("{}", rabbin_test(&BigUint::from(2305843009213693951u128), 5));
}
