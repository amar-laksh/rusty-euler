use std::time::Instant;

fn is_palindrome(number: i64) -> bool {
    let rev: i64 = (number.to_string().chars().rev().collect::<String>() ).parse().unwrap();
    if number != rev { return false; }
    return true;
}

fn largest_palindrome(limit: i64) -> i64 {
    let mut prod: i64;
    let mut i: i64 = limit;
    let mut j: i64;
    let mut largest: i64 = 0;
    while i > 899 {
        j = limit;
        while j > 899 {
            prod = i * j;
            if is_palindrome(prod) {
                largest = prod;
                break;
            }
            j -= 1;
        }
        if largest > 0 {break;}
        i -= 1;
    }
    return largest;
}

fn main() {
    let now = Instant::now();
    {
        println!("largest palindrome is: {}"
                , largest_palindrome(999));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}