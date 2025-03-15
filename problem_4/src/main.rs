fn prev_multiple(mut number: i64, divisor: i64, decrement: i64) -> i64 {
    let mut result: i64 = 0;
    while number > divisor {
        if number % divisor == 0 {
            result = number;
            break;
        }
        number -= decrement;
    }
    return result;
}

fn generate_ndigit(number: i64, mut n: u32) -> i64 {
    let mut result: i64 = number * i64::pow(10, n);
    while result % 10 == 0 {
        n -= 1;
        result = result + (number * i64::pow(10, n));
    }
    return result;
}

fn is_palindrome(number: i64) -> bool {
    let rev: i64 = (number.to_string().chars().rev().collect::<String>())
        .parse()
        .unwrap();
    if number != rev {
        return false;
    }
    return true;
}

fn largest_palindrome(len: u32) -> i64 {
    let limit: i64 = generate_ndigit(9, len);
    let (mut _prod, mut _i, mut _j) = (1, limit, limit);
    while _i > limit - i64::pow(10, len) {
        _j = limit;
        while _j > limit - i64::pow(10, len) {
            if is_palindrome(_i * _j) {
                println!("i, j: {},{}", _i, _j);
                _prod = _i * _j;
                break;
            }
            _j -= 2;
        }
        if _prod > 1 {
            break;
        }
        _i -= 2;
    }
    return _prod;
}

fn main() {
    println!("{}", largest_palindrome(2));
}
