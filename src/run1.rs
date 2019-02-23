// extern crate serde;
// extern crate serde_json;
// extern crate typescript_definitions;
// use serde::{Deserialize, Serialize};
use serde_json;
use failure::Error;

#[allow(unused)]
pub fn run() -> Result<(), Error> {
    use super::interface::*;
    // need the trait
    use typescript_definitions::TypeScriptifyTrait;

    let p = Point {
        x: 23,
        y: 24,
        z: 33,
    };
    let j = serde_json::to_string(&p)?;
    let f = FrontendMessage::Render {
        html: "stuff".into(),
        time: 33,
        other_result: Err(32),
    };
    let f2 = FrontendMessage::ButtonState {
        selected: vec!["a".into(), "b".into()],
        time: 33,
        other: None,
    };

    let b = MyBytes {
        buffer: vec![5u8, 6, 7, 8, 9, 186, 233],
    };
    println!("Point {:?}", p);
    println!("{}", j);
    println!("{}", serde_json::to_string(&f)?);
    println!("{}", serde_json::to_string(&f2)?);
    println!("{}", serde_json::to_string(&b)?);

    println!("{}", Point::type_script_ify());

    println!("{}", Enum::type_script_ify());
    println!("{}", FrontendMessage::type_script_ify());
    println!("{}", MyBytes::type_script_ify());
    println!("{}", MyBytes::type_script_guard().unwrap());

    // struct SB {
    //     buff : ::bytes::Bytes
    // }
    // let sb = SB { buff: ::bytes::Bytes::from(&b"this is a string"[..]) };
    // println!("{}", serde_json::to_string(&sb)?);
    use im::{vector, hashmap};

    let s2 : ::im::Vector<i32> = vector![1,2,3,4];
    
    println!("{}", serde_json::to_string(&s2)?);

    let s3 : ::im::HashMap<i32,&str> = hashmap!(1 => "s", 2 => "3");
    println!("{}", serde_json::to_string(&s3)?);

    Ok(())
}
