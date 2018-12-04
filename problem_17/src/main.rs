use std::time::Instant;
use std::collections::HashMap;

fn get_word(n: i64) -> String {
    let mut dict: HashMap<i64, &str> = HashMap::new();
    dict.insert(1,"one");dict.insert(2,"two");
    dict.insert(3,"three");dict.insert(4,"four");
    dict.insert(5,"five");dict.insert(6,"six");
    dict.insert(7,"seven");dict.insert(8,"eight");
    dict.insert(9,"nine");dict.insert(10,"ten");
    dict.insert(11,"eleven");dict.insert(12,"twelve");
    dict.insert(13,"thirteen");dict.insert(14,"fourteen");
    dict.insert(15,"fifteen");dict.insert(16,"sixteen");
    dict.insert(17,"seventeen");dict.insert(18,"eighteen");
    dict.insert(19,"nineteen");dict.insert(20,"twenty");
    dict.insert(30,"thirty");dict.insert(40,"fourty");
    dict.insert(50,"fifty");dict.insert(60,"sixty");
    dict.insert(70,"seventy");dict.insert(80,"eighty");
    dict.insert(90,"ninty");dict.insert(100,"onehundred");
    dict.insert(200,"twohundred");dict.insert(300,"threehundred");
    dict.insert(400,"fourhundred");dict.insert(500,"fivehundred");
    dict.insert(600,"sixhundred");dict.insert(700,"sevenhundred");
    dict.insert(800,"eighthundred");dict.insert(900,"ninehundred");
    dict.insert(1000,"onethousand");
    dict.insert(1001,"and");

    if n.to_string().len() == 1 {
        return dict.get(&n).unwrap().to_string();
    }
    else if n.to_string().len() == 2 {
        if n % 10 == 0 {
            return dict.get(&n).unwrap().to_string();
        }
        if n < 20 {
            return dict.get(&n).unwrap().to_string();
        }
        else {
            let tens: i64 = (n / 10 as i64)*10;
            let ones: i64 = n % 10;
            let first: String = dict.get(&ones).unwrap().to_string().to_owned();
            let mut second: String = dict.get(&tens).unwrap().to_string().to_owned();
            second.push_str(&first);
            return second;

        }
    }
    else if n.to_string().len() == 3 {
        if n % 100 == 0 {
            return dict.get(&n).unwrap().to_string();
        }
        else{
            let hundreds: i64 = (n / 100)* 100;
            let others: i64 = n % 100;
            let mut first: String = dict.get(&hundreds).unwrap().to_string().to_owned();
            let second: String = get_word(others).to_owned();
            let and: String = dict.get(&1001).unwrap().to_string().to_owned();;
            first.push_str(&and);
            first.push_str(&second);
            return first;
        }
    }
    return dict.get(&1000).unwrap().to_string();
}

fn sum_letters(n: i64) -> i64 {
    let mut s: i64 = 0;
    for i in 1..n+1 {
        s += get_word(i).len() as i64;
    }
    return s;
}



fn main() {
    let now = Instant::now();
    {
        println!("The word is: {}"
                , sum_letters(1000));
    }
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64)
                + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Time taken: {} seconds", sec);
}