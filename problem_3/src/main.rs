fn is_prime(number: i64) -> bool {
    let mut i: i64 = 3;
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    while i < ((number as f64).sqrt() as i64 + 1) {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn largest_prime_factor(limit: i64) -> i64 {
    let (mut i, mut prime) = (2, 0);
    while i < limit {
        if limit % i == 0 && is_prime(i) {
            prime = i;
            break;
        }
        i = if i == 2 { 3 } else { i + 2 };
    }
    if prime == 0 {
        return limit;
    } else {
        let new_limit = limit / prime;
        largest_prime_factor(new_limit)
    }
}

fn main() {
    println!("{}", largest_prime_factor(600851475143));
}
