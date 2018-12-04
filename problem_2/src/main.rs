use std::time::Instant;
fn sum_fab(limit: i64) -> i64{
    let mut sum: i64 = 0;
    let mut a: i64 = 1;
    let mut b: i64 = 2;
    let mut c: i64;
    let mut d: i64 = 2;
    while d < limit {
        sum += d;
        c = a + (2*b);
        d = (2*a) + (3*b);
        b = d;
        a = c;
    }
    return sum;
}

fn main() {
    let now = Instant::now();
    {
        println!("The sum of even valued terms of fabonacci: {}"
                , sum_fab(4000001));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}