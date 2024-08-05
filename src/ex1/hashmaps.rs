use std::collections::HashMap;

pub fn run(){
    let mut person:HashMap<&str, i32>= HashMap::new();
    person.insert("a", 32);
    person.insert("b", 12);
    person.insert("c", 52);

    println!("the age is {:?}", person.get("a").unwrap());

    if person.contains_key("c"){
        println!("value exists");
    }else{
        println!("value does not exists");
    }

    match person.get("a"){
        Some(v) => {
            println!("value exists");
        }
        None => {
            println!("value does not exists");
        }
    }

    for (name, age) in &person{
        println!("the person {name} is {age} years old");
    }

    let mut likes:HashMap<&str, &str>= HashMap::new();
    likes.insert("a", "apple");
    likes.insert("a", "mango");

    likes.entry("a").or_insert("orange");
    println!("the person likes {:?}", likes);
}


pub fn run2(){
    let some_vec = vec![5, 5, 8, 8, 1, 0, 1];
    let mut freq_vec:HashMap<i32, u32> = HashMap::new();

    for v in &some_vec{
        let a = freq_vec.entry(*v).or_insert(120);
        *a += 1;
    }


    // for (e, v) in freq_vec{
    //     println!("{e} has frequency {v}");
    // }

    println!("{:?}", freq_vec);
}


