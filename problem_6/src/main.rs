use std::time::Instant;



fn difference(limit: i64) -> i64 {
    let mut sum: i64 = ( limit * (limit + 1) ) / 2;
    let sum_sq: i64 = ( limit * (limit + 1) * ( (2 * limit) + 1) ) / 6;
    sum = sum.pow(2);
    return sum - sum_sq;
}

fn main() {
    let now = Instant::now();
    {
        println!("The difference is: {}"
                , difference(100));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}