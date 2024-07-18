pub fn run(){
    let name="ns";
    let mut age:u8=12;
    age+=12;
    println!("{:?}",(12, name, age) ); // debug args

    let (a, b) = ("ns", 32);
    println!("{:?}",(a,b) ); // debug args

}