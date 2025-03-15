fn sum_evenfib(limit: i64) -> i64 {
    let (mut sum, mut a, mut b, mut _c, mut d) = (0, 1, 2, 0, 2);
    while d < limit {
        sum += d;
        _c = a + (2 * b);
        d = (2 * _c) - b;
        b = d;
        a = _c;
    }
    return sum;
}

fn main() {
    println!("{}", sum_evenfib(4000001));
}
