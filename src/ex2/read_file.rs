use std::fs;

#[derive(Debug)]
struct Name(String);

fn read(){

}


pub fn run()
{
    let path= "C:\\Users\\nsdev\\code\\rust_proj\\rust3\\p001_rust_exercises\\poem.txt";
    println!("{:?}", path);
    let contents= fs::read_to_string(path).expect("should have read file");
    println!("contents\n{contents}");

    let n = Name("abcd".to_string());
    println!("{:?}", n);
}