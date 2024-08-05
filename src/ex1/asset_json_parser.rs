use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
use std::time::Instant;

use serde_json::{from_str, to_string};
use std::{collections::HashMap, env, string};

//////////////////////////////////////////////////////////////////////////////
///
///     structs for room from graphql request
///
//////////////////////////////////////////////////////////////////////////////

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    data: Assets,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Assets {
    assets: Vec<Asset>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Asset {
    id: String,
    commitId: String,
    typeId: String,
    applicationId: String,
    roomId: String,
    properties: Option<Vec<Properties>>,
    location: Option<Location>,
    speckleReference: Option<SpeckleReference>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct Properties {
    key: String,
    metadata: Option<String>,
    value: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct Location {
    units: String,
    x: f32,
    y: f32,
    z: f32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct SpeckleReference {
    modelId: Option<String>,
    ObjectId: Option<String>,
    projectId: Option<String>,
    serverURL: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Argument {
    commitId: String,
}

pub fn run() {
    let now = Instant::now();
    let file_path = "C:\\Users\\nsdev\\code\\rust3\\p001_rust_exercises\\asset.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
    let deser = from_str::<Data>(&contents);
    if deser.is_ok() {
        // println!("{:#?}", deser.ok().unwrap());
        let a = deser.ok().unwrap();        
        test_json(a);
    } else {
        println!("error in deser\n{:#?}", deser.err());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

pub fn test_json(inp: Data) {
    println!("{:?}", inp);

    // let vals = inp.keys();
    // let n = vals.len();
    // println!("num of data {}", n);

    // for v in vals.iter() {
    //     let g = &v.geometry;
    //     let n = v.name.clone();
    //     match g {
    //         Some(_x) => {
    //             println!("room name with geometry {:?}", n)
    //         },
    //         _ => println!("room name without geometry {:?}", n),
    //     };
    // }
}
