use serde_json;
use serde_json::Result;
use serde_json::{Map, Value};
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    fname: String,
    lname: String,
}

pub fn run() {
    let path = "./test.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("{}", res);

    let p: Vec<Person> = serde_json::from_str(&data).unwrap();
    println!("Name: {} {}", p[0].fname, p[0].lname);
    for x in p.iter() {
        println!("num: {:?}", x);
    }
}
