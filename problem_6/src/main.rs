fn difference(limit: i64) -> i64 {
    let mut sum: i64 = (limit * (limit + 1)) / 2;
    let sum_sq: i64 = (limit * (limit + 1) * ((2 * limit) + 1)) / 6;
    sum = sum.pow(2);
    return sum - sum_sq;
}

fn main() {
    println!("{}", difference(100));
}
