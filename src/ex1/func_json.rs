extern crate serde_json;

use serde_json::Value;

pub fn run() {
    let data: Value = serde_json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
    println!("data: {:?}", data);
    // data: {"bar":"baz","foo":13}
    println!("object? {}", data.is_object());
    // object? true

    let obj = data.as_object().unwrap();
    let foo = obj.get("foo").unwrap();

    println!("array? {:?}", foo.as_array());
    // array? None
    println!("u64? {:?}", foo.as_u64());
    // u64? Some(13u64)

    for (key, value) in obj.iter() {
        println!(
            "{}: {}",
            key,
            match *value {
                Value::Number(ref v) => format!("{} (number)", v),
                Value::String(ref v) => format!("{} (string)", v),
                _ => format!("other"),
            }
        );
    }
    // bar: baz (string)
    // foo: 13 (u64)
}
