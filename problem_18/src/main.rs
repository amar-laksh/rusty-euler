fn total_rows(length: usize) -> i64 {
    let result = (((4 * (length + length) + 1) as f64).sqrt() as i64 - 1) / 2;
    return result;
}
fn list(n_index: usize, row: i64, array: &Vec<i64>) -> Vec<i64> {
    let max = row + 2;
    let mut max_i = 0;
    let mut max_v = 0;
    for (i, &v) in array.iter().skip(n_index).enumerate() {
        let array_index = i + n_index;
        let diff = ((array_index - n_index) as i64).abs();
        //        println!("diff: {}, max: {}, i: {}, v: {}", diff, max , array_index, v);
        if max - 1 <= diff && diff <= max {
            if v > max_v {
                max_v = v;
                max_i = array_index as usize;
            }
        }
    }
    return vec![max_v, max_i as i64];
}

fn triangle() -> i64 {
    //    let t = vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3];
    let t = vec![
        75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34, 88, 02,
        77, 73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40, 80, 70, 33, 41,
        48, 72, 33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 70, 11, 33,
        28, 77, 73, 17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 63,
        66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 04, 62, 98, 27, 23, 09, 70, 98, 73, 93,
        38, 53, 60, 04, 23,
    ];
    let mut rows: i64 = total_rows(t.len());
    println!("length: {}, rows: {}", t.len(), rows);
    let mut c = 0;
    let mut sum: i64 = t[0];
    let mut result;
    for (i, _) in t.iter().enumerate() {
        for _ in 0..i + 1 {
            result = list(c, rows as i64, &t);
            //            println!("value of c : {},  list: {:?}, row: {}", c, result, row);
            //            println!("");
            c = result[1] as usize;
            rows += 1;
            sum += result[0];
            if result[0] == 0 && result[1] == 0 {
                return sum;
            }
        }
    }
    return sum;
}

fn main() {
    println!("{}", triangle());
}
