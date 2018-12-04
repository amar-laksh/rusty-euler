use std::time::Instant;


fn b_mul(n: i64, k: i64) -> i64 {
    let mut p: i64 = 1;
    for i in 1..k+1 {
        p *= n - k + i;
        p /= i;
    }
    return p;
}


fn main() {
    let now = Instant::now();
    {
        println!("The number is: {}", b_mul(40, 20));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}