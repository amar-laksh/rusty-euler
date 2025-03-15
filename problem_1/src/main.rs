fn sum_multiples(multiple: i64, limit: i64) -> i64 {
    let count = limit / multiple;
    return multiple * count * (count + 1) / 2;
}

fn main() {
    let answer = sum_multiples(3, 999) + sum_multiples(5, 999) - sum_multiples(15, 999);
    println!("{}", answer);
}
