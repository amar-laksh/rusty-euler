use std::time::Instant;

fn prime_sieve() -> [bool; 2_000_000] {
    const SIZE: usize = 2_000_000;
    let mut slots: [bool; SIZE] = [true; SIZE];
    slots[0] = false;
    slots[1] = false;

    // We calculate the primes using a simple stride and marking off multiples 
    for stride in 2..(SIZE/2) {
        let mut pos = stride;
        while pos < (SIZE - stride) {
            pos += stride;
            slots[pos] = false;
        }
    }
    return slots;
}

fn sum_prime() -> usize{
    let mut sum: usize = 0;
    for (idx, pr) in prime_sieve().into_iter().enumerate() {
        if *pr { sum += idx; }
    }
    return sum;
}



fn main() {
    let now = Instant::now();
    {
        println!("The sum of prime numbers is: {}"
                ,sum_prime());
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}