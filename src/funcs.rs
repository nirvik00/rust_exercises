pub fn run(){
    greet ("hi", "ns");
    let sum= add(10, 21);
    println!("sum= {}", sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1:i32,  n2:i32| n1+n2 + n3;
    println!("C sum {}", add_nums(2,3));  
}

pub fn greet (g: &str, name: &str) {
    println!("{},  {}", g, name);
}

fn add(n1:i32, n2:i32) -> i32 {
    n1 + n2
}