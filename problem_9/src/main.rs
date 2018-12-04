
use std::time::Instant;

fn triplet(value: i64) -> Vec<i64> {
    let mut a: i64 = 0;
    let mut b: i64;
    let mut c: i64;
    while a < value / 3 {
        b = a;
        while b < value / 2 {
            c = value - a - b;

            if ((a*a) + (b*b)) == c*c {
                return vec![a, b, c];
            }
            b += 1;
        }
        a += 1;
    }
    return vec![0, 0, 0];
}

fn main() {
    let now = Instant::now();
    {
        let triplet: Vec<i64> = triplet(1000);
        println!("The largest product is: {}"
                , triplet[0]*triplet[1]*triplet[2]);
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}