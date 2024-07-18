pub fn run() {
    let s1: String = String::from("hello");
    {
        let s2: String = s1.clone() + " world";
        println!("s2 is {}", s2);
    }
    println!("s1 is {}", s1);

    {
        let v: Vec<i32> = vec![1, 2, 3];
        takes_ownership(v.clone());
        println!("vec is {:?}", v);
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

fn takes_ownership(vec: Vec<i32>) {
    println!("move ownership vec is {:?}", vec);
}

fn give_ownership() -> Vec<i32> {
    vec![1, 2, 4]
}

fn combined_ownership(mut v:Vec<i32>) -> Vec<i32>
{
    v.push(43);
    v
}
