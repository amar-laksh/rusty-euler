use std::time::Instant;
fn is_divisible(n: i64, limit: i64) -> bool {
    let mut i: i64 = 1;
    while i < limit {
        if n % i != 0 { return false; }
        i += 1;
    }
    return true;
}

fn smallest_divisible(limit: i64) -> i64 {
    let mut i: i64 = limit * limit;
    loop {
            if is_divisible(i, limit) {
                return i;
            }
        i += limit;
    }
}

fn main() {
    let now = Instant::now();
    {
        println!("smallest number divisible is: {}"
                , smallest_divisible(20));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}