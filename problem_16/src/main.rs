extern crate num;

use std::time::Instant;
use num::{BigUint};
use num::bigint::{ToBigUint};


fn pow(base: &BigUint, exp: usize) -> BigUint {
    return (1..exp+1).fold(1.to_biguint().unwrap(), |acc, _| acc * base)
}

fn sum_of_power(n: i64) -> u32{
     let base = 2.to_biguint().unwrap();
    let pow = pow(&base, n as usize);
    let as_string = format!("{}", pow);
    let chars: Vec<char> = as_string.chars().collect();
    let sum = chars.iter().map(|x| x.to_digit(10).unwrap()).fold(0, |acc, cur| acc + cur);
    return sum;
}

fn main() {
    let now = Instant::now();
    {
        println!("The number is: {}", sum_of_power(1000));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}