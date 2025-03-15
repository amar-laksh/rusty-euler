fn is_divisible(n: i64, limit: i64) -> bool {
    let mut i: i64 = if n % 2 == 0 { 3 } else { 2 };
    while i < limit {
        if n % i != 0 {
            return false;
        }
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
    println!("{}", smallest_divisible(20));
}
