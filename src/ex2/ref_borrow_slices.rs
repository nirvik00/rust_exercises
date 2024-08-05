pub fn run()
{
    let x=10;

    let mut y=&x;
    println!("{:?}", &x);
    y =  &32;
    println!("{:?}", &y);

    let z: &&i32 = &y;
    println!("{:?}", &z);

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("word length {:?}", word);
    s.clear(); // this empties the String, making it equal 


    let s2 = String::from("this is good");
    println!("{:?}", &s2[0..5]);
    println!("{:?}", &s2[3..s2.len()]);
    println!("{:?}", &s2[0..s2.len()]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}