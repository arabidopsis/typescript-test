use serde_json;
use serde_json::Error;
use std::collections::HashMap;

macro_rules! addit2 {
    ($hm:ident, $e:expr) => {
        let s = serde_json::to_string(&$e)?;
        let k = stringify!($e);
        $hm.insert(k.into(), s);
    };
}

trait Addit {}

pub fn run() -> Result<(), Error> {
    use super::interface::*;
    let mut hm: HashMap<String, String> = HashMap::new();

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

    println!("{}", serde_json::to_string(&hm)?);

    Ok(())
}
