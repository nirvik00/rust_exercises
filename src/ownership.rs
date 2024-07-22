pub fn run() {
    let s1: String = String::from("hello");
    {
        let s2: String = s1.clone() + " world";
        println!("s2 is {}", s2);
    }
    println!("s1 is {}", s1);

    {
        let mut v: Vec<i32> = vec![1, 2, 3];
        takes_ownership(&mut v);
        println!("updated vec after move is {:?}", &v);
    }

    {
        let v: Vec<i32> = give_ownership();
        println!("take ownership {:?}", v);
    }

    {
        let v = vec![1, 3, 4, 6, 7];
        let x = combined_ownership(v.clone());
        println!("{:?}", v);
        println!("{:?}", x);
    }
}

fn takes_ownership(vec: &mut Vec<i32>) {
    vec.push(12);
    println!("move ownership vec is {:?}", vec);
}

fn give_ownership() -> Vec<i32> {
    vec![1, 2, 4]
}

fn combined_ownership(mut v: Vec<i32>) -> Vec<i32> {
    v.push(43);
    v
}

pub fn f2() {
    let x = 12;
    let mut y = x;
    y = 32;
    println!("{:?}", y);
    println!("{:?}", x);
}

pub fn f3() {
    let s1 = {
        let s1 = generate_str();
        s1
    };

    let s2=s1;
    println!("{:?}", s2);
}

pub fn generate_str() -> String {
    let s = String::from("Hello world!");
    s
}
