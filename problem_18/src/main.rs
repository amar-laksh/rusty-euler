use std::time::Instant;
/* check if len of vector is triangle number if yes

*/

fn T(n: i64) -> i64 {
    let mut m: i64 = (((((8*n + 1) as f64).sqrt() as i64 - 1)/ 2) as f64).floor() as i64;
    return n - (m*(m+1))/2 + 1;
}

fn triangle() {
    /*
     3 7 4 2 4 6 8 5 9 3
     0 0 1 0 1 2 0 1 2 3
     0 1 2 3 4 5 6 7 8 9

     index  list_ind  c   c < index      list_ind - c <= 1  stride  max
     0      0         0   true            true                       3
     0      0         1   false                              [3]
     1      0         0   true            true                       7
     2      1         1   true            true                       7     
     3      0         2   false                              [3,7]
     2      1         0   true            true                       4 

     

    */
    let small_t: [i64; 10] =
    [3, 7, 4, 2, 4, 6, 8, 5, 9, 3];
    let mut indexes: Vec<i64> = vec![];
    for n in 0..10 {
        indexes.push(T(n) - 1);
    }
    // let large_t: Vec<i64> =
    // vec![75, 95, 64, 17, 47, 82, 18, 35
    //     , 87, 10, 20, 04, 82, 47, 65, 19
    //     , 01, 23, 75, 03, 34, 88, 02, 77
    //     , 73, 07, 63, 67, 99, 65, 04, 28
    //     , 06, 16, 70, 92, 41, 41, 26, 56
    //     , 83, 40, 80, 70, 33, 41, 48, 72
    //     , 33, 47, 32, 37, 16, 94, 29, 53
    //     , 71, 44, 65, 25, 43, 91, 52, 97
    //     , 51, 14, 70, 11, 33, 28, 77, 73
    //     , 17, 78, 39, 68, 17, 57, 91, 71
    //     , 52, 38, 17, 14, 91, 43, 58, 50
    //     , 27, 29, 48, 63, 66, 04, 68, 89
    //     , 53, 67, 30, 73, 16, 69, 87, 40
    //     , 31, 04, 62, 98, 27, 23, 09, 70
    //     , 98, 73, 93, 38, 53, 60, 04, 23];

    let mut c: i64 = 0;
    for (row, index) in small_t.iter().zip(indexes.iter_mut()) {
        while c < index + 1 &&  {

        } 
        println!("{}:{}", row, index);
    }
}

fn main() {
    let now = Instant::now();
    {
        println!("The word is: {:?}"
                , triangle());
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}