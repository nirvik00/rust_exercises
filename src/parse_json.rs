use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
use std::time::{Instant};

use serde_json::{from_str, to_string};
use std::{collections::HashMap, env, string};


///
#[derive(Deserialize, Serialize, Debug)]
pub struct Geometry {
    r#type: String,
    coordinates: Vec<Vec<Vec<f32>>>,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Elevation {
    r#unit: String,
    r#value: f32,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Level {
    r#name: String,
    elevation: Elevation,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Area {
    r#unit: String,
    value: f32,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Val {
    name: String,
    geometry: Option<Geometry>,
    level: Level,
    area: Area,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct ValVec {
    values: Vec<Val>,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Rooms {
    rooms: ValVec,
}

///
#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    data: Rooms,
}

pub fn run() {
    let now = Instant::now();
    
    // let file_path = "C:\\Users\\nsdev\\code\\rust3\\p001\\test.json".to_owned();
    let file_path = "C:\\Users\\nsdev\\code\\rust3\\p001\\roomoutput_postman.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    let deser = from_str::<Data>(&contents);
    if deser.is_ok() {
        // println!("{:#?}", deser.ok().unwrap());
        let a = deser.ok().unwrap();
        test_json(a);
    } else {
        println!("{:#?}", deser.err());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

pub fn test_json(inp: Data) {
    let vals = inp.data.rooms.values;
    let n = vals.len();
    println!("num of data {}", n);

    for v in vals.iter() {
        let g = &v.geometry;
        let n = v.name.clone();
        match g {
            Some(_x) => {
                println!("room name with geometry {:?}", n)
            },
            _ => println!("room name without geometry {:?}", n),
        };
    }
}
