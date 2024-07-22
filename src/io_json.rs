// cargo add serde
// cargo add serde --features derive
// cargo add serde_json


use serde::{Deserialize, Serialize}; // macros

use serde_json::{to_string, from_str}; // will allow serialization


#[derive(Serialize, Deserialize, Debug)]// annotate custom data structures
struct Dog{
    name: String,
    year_born: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct DogOwner{
    first_name: String,
    last_name:String,
}

pub fn run() 
{   
    dog_serialization();
    dog_deserialization();
}

fn dog_serialization()
{
    let owner01 = DogOwner{first_name:"abc".to_string(), last_name:"def".to_string()};
    let dog01 = Dog{name:"gbdb".to_string(), year_born:2020, owner: owner01};
    let dog_ser = to_string(&dog01);
    if dog_ser.is_ok()
    {
        println!("{}", dog_ser.ok().unwrap());
    }else{
        println!("{:#?}", dog_ser.err());
    }
}

fn dog_deserialization()
{
    let json_str = r#"
    {"name":"gbdb","year_born":2020,"owner":{"first_name":"abc","last_name":"def"}}
    "#;
    let dog_deser = from_str::<Dog>(json_str);
    if dog_deser.is_ok()
    {
        println!("{:#?}", dog_deser.ok().unwrap());
    }
    else
    {
        println!("{:#?}", dog_deser.err());
    }
}


