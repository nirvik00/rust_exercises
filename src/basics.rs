pub fn run()
{
    println!("Hello, world!");
    let x: i32 = 10;
    println!("x is {x}");

    type Age = u8;
    let peter_age: Age = 43;
    println!("{}", peter_age);

    let a: i32 = 10;
    let _b: f32 = a as f32;

    multiplication(12, a);

    // compund data types
    let _fixed_str: &str = "Fixed string length";
    let mut _flexible_str: String = String::from("abcd");

    let mut _arr1: [i32; 5] = [4, 5, 6, 7, 8];
    
    let _v1: Vec<i32> = vec![4, 5, 6, 7, 8];

    let _my_info = ("Salary", 40000, "Age", 40);
    let _sal = _my_info.1;
    let _age = _my_info.3;

    my_fn("abcd");
    let s2:&str = "this is somethinf";
    my_fn(s2);

    {
        let a:i32 = 12;
        let b:i32 = 12;
        let c = f2(a, b);
        println!("\nresult {:?}", c);

        let (d,e,f) = f2(a, b);
        println!("{}, {}, {}", d, e, f);

    }

    let full_name = {
        let fname = "abcd";
        let lname = "efgh";
        format!("{fname} {lname}");
    };
    println!("{:?}", full_name);
}



fn my_fn(s: &str)
{
    print!("{}", s)
}

fn multiplication(a:i32, b:i32) -> i32 
{   
    let c = a*b;
    println!("{}", c);
    c
}

fn f2(a:i32, b:i32) -> (i32, i32, i32)
{
    (a*b, a+b, a-b)
}