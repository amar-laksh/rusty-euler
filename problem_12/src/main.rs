use std::time::Instant;

fn is_prime(number: i64) -> bool {
    let mut i: i64 = 3;
    if number == 2 { return true; }
    if number % 2 == 0 { return false; }
    while i < number {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn magic(primes: &mut Vec<i64>) -> i64 {
    let mut prev: i64 = primes[0];
    let mut c: i64 = 1;
    let mut counts: Vec<i64> = vec![];
    for i in primes {
        if *i == prev {
            c += 1;
        }
        else {
            counts.push(c);
            c = 2;
            prev = *i;
        }
    }
    counts.push(c);
    return counts.iter().product();
}

fn no_of_factors(limit: i64, mut primes: &mut Vec<i64>) -> i64 {
    let mut i: i64 = 2;
    let mut prime: i64 = 0;
    while i < limit {
        if limit % i == 0 && is_prime(i){
            primes.push(i);
            prime = i;
            break;
        }
        i = if i == 2 { 3 } else { i + 2 };
    }
    if prime == 0 { primes.push(i); return magic(primes); }
    else {
        let new_limit = limit/prime;
        no_of_factors(new_limit, &mut primes)
    }
}
fn tri(n: i64) -> i64 {
    return (n * (n + 1))/2;
}

fn triangle_n(value: i64) -> i64 {
    let mut primes: Vec<i64>;
    let mut n: i64 = 1;
    let mut number: i64;
    loop {
        primes = vec![];
        number = tri(n);
        if no_of_factors(number, &mut primes) > value {
            return number;
        }
        n += 1;
    }
    }
fn main() {
    let now = Instant::now();
    {
        println!("The Triangle number is: {:?}"
                , triangle_n(500));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}