use std::time::Instant;

fn is_prime(n: i64) -> bool {
    let mut i: i64 = 3;
    if n == 1 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    while i < n {
        if n % i == 0 {return false;}
        i += 2;
    }
    return true;
}


fn nth_prime_simple(nth: i64) -> i64 {
    let mut i: i64 = 3;
    let mut c: i64 = 1;
    loop {
        if is_prime(i) { c += 1; }
        if nth == c { return i; }
        i += 2;
    }
}

fn phi(x: i64, a: i64) -> i64 {
    if x == 0 { return 0;}
    if a == 1 {
        if x % 2 != 0 { return (x+1)/2; }
        else { return x/2 ; }
    }
    let result: i64 = phi(x, a - 1) - phi(x / nth_prime_simple(a), a - 1);
    return result;
}

fn pi(x: i64) -> i64 {
    if x <= 2 { return 1; }
    let result: i64;
    let temp: i64 = pi((x as f64).sqrt() as i64);
    result = phi(x, temp) + temp  - 1;
    return result;
}

fn nth_prime(nth: i64) -> i64 {
    let temp_nth: i64 = (nth * (nth as f64).log(5.0) as i64) * 2;
    let mut c: i64 = pi(temp_nth);
    let mut i: i64 = if temp_nth % 2 == 0 { temp_nth + 1 } else { temp_nth };
    while c <= nth {
        if is_prime(i) {
            c += 1;
            if c == nth { return i; }
        }
        i += 2;
    }
    return 0;
}

fn main() {
    let now = Instant::now();
    {
        println!("The 10001st prime number is: {}"
                , nth_prime(10001));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}