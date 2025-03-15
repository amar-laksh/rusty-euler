fn b_mul(n: i64, k: i64) -> i64 {
    let mut p: i64 = 1;
    for i in 1..k + 1 {
        p *= n - k + i;
        p /= i;
    }
    return p;
}

fn main() {
    println!("{}", b_mul(40, 20));
}
