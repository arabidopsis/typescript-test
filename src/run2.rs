use serde_json;
use failure::Error;
use std::collections::HashMap;
use std::borrow::Cow;
use regex::Regex;
    

macro_rules! addit2 {
    ($hm:ident, $e:expr) => {
        let re = Regex::new(r"^\w+").unwrap();
        let json = serde_json::to_string(&$e)?;
        let k = stringify!($e);
        let f = re.find(k).unwrap().as_str();
        $hm.push((k.into(),f.into(),json));
    };
}

pub fn run() -> Result<(), Error> {
    use super::interface::*;
    let mut hm: Vec<(String,String,String)> = vec![];

    addit2! {hm, FrontendMessage::ButtonState {
        selected: vec!["a".into(), "b".into()],
        time: 33,
        other: None,
    }}

    addit2! {hm, FrontendMessage::Render {
        html: "<html/>".into(),
        time: 23656,
        other_result: Ok("done"),
    }
    }

    addit2! {hm, FrontendMessage::Stuff {
            borrow: vec![4,5,6]
        }
    }

    addit2! {hm, FrontendMessage::Init {
            id : "myid".into()
        }
    }

    addit2! {hm, Point {
        x:1, y:2, z:3
    }}

    use std::iter::FromIterator;
    let v = [("a", 32), ("b", 33)];
    let map = HashMap::<&str,i32>::from_iter(v.iter().cloned());
    addit2! {hm, Borrow {
            raw: "raw",
            cow: Cow::Borrowed("raw"),
            map: map,
            array: vec!["a".into(), "b".into()]
        }
    }
    addit2! {hm, IntMap {
            intmap: HashMap::from_iter([(32,32), (9999,666)].iter().cloned())
        }
    }

    addit2! {hm, S::F(5, 32.5, "ssss".into())}

    addit2! {hm,  Search {results: Err("nothing".into()) } }
    
    let addr = Address { number: 32, street: "x way".into(), zip: 202021 };
    addit2! {hm,  Search {results: Ok(vec![Record {name :"me". into(), 
            address: addr.clone(), year:Some(1942)}])} }

    
    addit2! {hm, DependsOnValue {
        value: vec![Value { value: 22}, Value { value: 33}, Value {value: 66}]
    }}

    addit2! {hm, TyEnum::Red}

    addit2! {hm, MyBytes {
        buffer: vec![5u8, 6, 7, 8, 9, 186, 233, 55,66],
    }}

    addit2! {hm, Chrono { datetime: chrono::Local::now(), 
            duration: std::time::Duration::new(22,3),
            systime: std::time::SystemTime::now(),
            path: std::path::PathBuf::from("/a/b/c/d.txt"),
            dt:  chrono::Utc::now(),
            
            }}
    addit2! {hm, Either { either: either::Either::Left(addr.clone()) }}

    println!("{}", serde_json::to_string(&hm)?);

    Ok(())
}
