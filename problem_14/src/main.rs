fn _hailstone(mut n: i64) -> i64 {
    let mut p: i64;
    let mut c: i64 = 1;
    loop {
        p = if n % 2 == 0 { n / 2 } else { ((3 * n) + 1) / 2 };
        if p == 1 {
            c += 1;
            break;
        }
        n = p;
        c += 1
    }
    return c;
}

fn series(limit: i64) -> i64 {
    let mut i: i64 = 1;
    let mut max: i64 = 1;
    let mut steps: i64;
    let mut number: i64 = 0;
    while i < limit {
        steps = _hailstone(i);
        if steps > max {
            max = steps;
            number = i;
        }
        i += 2;
    }
    return number;
}

fn main() {
    println!("{}", series(1_000_000));
}
