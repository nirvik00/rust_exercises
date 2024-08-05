use std::collections::HashMap;

//
fn check_and_divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn divide(a: i32, b: i32) {
    let quotient = check_and_divide(a, b);
    match quotient{
        None=> println!("none"),
        Some(answer)=>{
            println!("quotient = {:?}", Some(answer).unwrap())
        }
    }
}

pub fn run() {
    divide(4, 2);
    let x = Some(5);
    println!("{}", x.unwrap());

    let mut scores =HashMap::new();
    scores.insert(String::from("blue"),23);
    scores.insert(String::from("red"),213);

    for e in &scores{
        println!("{:?}", e);
    }

    for (k, v) in &scores{
        println!("k={:?}, v={:?}", k, v);
    }
}



