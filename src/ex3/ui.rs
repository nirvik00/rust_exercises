use std::io;
use std::io::*;

pub fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    println!("{}", input);
}