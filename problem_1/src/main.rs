use std::cmp;
use std::time::Instant;
fn sum(numbers: Vec<i32>, limit: i32) -> i32{
    let mut sum: i32 = 0;
    let (first, second)  = (numbers[0], numbers[1]);
    let mut i: i32 = cmp::min(first, second);
    while i < limit {
        if(i % first == 0) || (i % second == 0){
            sum += i;
        }
        i += 1;
    }
    return sum;
}

fn main() {
    let now = Instant::now();
    {
        println!("The sum of 3 or 5 multiples : {}"
                , sum(vec![3,5], 1000));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}