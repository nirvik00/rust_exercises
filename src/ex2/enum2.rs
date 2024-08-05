enum IpAddrKind{
    v4, 
    v6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn run()
{
    let c=Coin::Penny;
    let x = value_in_cents(c);
    println!("value of a dime is {:?}", x);
    //
    let vec =vec![1, 2, 3, 4];
    let third = &vec[2];
    println!("some number at pos 2 is {:?}", third);
    let third2 = vec.get(123);
    match third2{
        Some(num) => {
            println!("some number at pos 2 is {:?}", num);
        },
        None => {
            println!("error at pos 123");
        }
    }

    let s = "hello";
    for c in s.chars(){
        println!("first char is {:?}", c);
    }
    println!("string is {:?}", &s.chars());    

}