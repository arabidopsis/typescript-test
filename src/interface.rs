#![allow(unused)]

use serde::Serialize;
use typescript_definitions::{TypeScriptify, TypescriptDefinition};
// if you only want to generate ts then you
// can uncomment the next line.

// #[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Serialize, TypescriptDefinition)]
pub struct Newtype(i64);

#[derive(Serialize, TypescriptDefinition, TypeScriptify, Debug)]
pub struct Point {
    #[serde(rename = "X")]
    pub x: i64,
    #[serde(rename = "Y")]
    pub y: i64,
    pub z: i64,
}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub enum Enum {
    #[allow(unused)]
    V1 {
        #[serde(rename = "Foo")]
        foo: bool,
    },
    #[allow(unused)]
    V2 {
        #[serde(rename = "Bar")]
        bar: i64,
        #[serde(rename = "Baz")]
        baz: u64,
    },
    #[allow(unused)]
    V3 {
        #[serde(rename = "Quux")]
        quux: String,
    },
}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
// #[typescript(isa(T="isa_what"))]
pub struct Value<T> {
    pub value: T,
}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct DependsOnValue {
    pub value: Vec<Value<i32>>,
}

#[derive(TypescriptDefinition, Serialize, TypeScriptify)]
#[serde(tag = "tag", content = "fields")]
#[typescript(guard = "true")]
/// This is some API Event.
pub enum FrontendMessage {
    Init {
        id: String,
    },
    ButtonState {
        selected: Vec<String>,
        time: u32,
        other: Option<String>,
    },
    Render {
        html: String,
        time: u32,
        other_result: Result<&'static str, i32>,
    },
    Stuff {
        borrow: Vec<i32>,
    },
}

use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
// #[typescript(verify)]
pub struct Borrow<'a> {
    pub raw: &'a str,
    pub cow: Cow<'a, str>,
    pub map: HashMap<&'a str, i32>,
    pub array: Vec<String>,
}
#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct IntMap {

    pub intmap: HashMap<i32, i32>,

}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct MyBytes {
    #[serde(serialize_with = "typescript_definitions::as_byte_string")]
    pub buffer: Vec<u8>,
}
#[derive(Serialize, TypescriptDefinition)]
#[serde(tag = "kind", content = "fields")]
pub enum S {
    A,
    E2 {
        key: i32,
        a: i32,
        #[serde(skip)]
        b: f64,
    },
    F(i32, #[serde(skip)] f64, String),
    #[serde(skip)]
    Z,
}

#[derive(Serialize, TypescriptDefinition, Clone)]
pub struct Address {
    pub number: i32,
    pub street: String,
    pub zip: i32,
}

#[derive(Serialize, TypescriptDefinition)]
pub struct Record {
    pub name: String,
    pub  address: Address,
    pub year: Option<i32>,
}
#[derive(Serialize, TypescriptDefinition)]
pub struct Search {
    #[typescript(array_check = "first")]
    pub results: Result<Vec<Record>, String>,
}

#[derive(Serialize, TypescriptDefinition)]
pub enum TyEnum {
    Red,
    Green,
    Blue,
}


#[derive(Serialize, TypescriptDefinition)]
pub struct Value2<T> {
    #[typescript(user_type_guard=true)]
    pub value: T,
}

#[derive(Serialize, TypescriptDefinition)]
pub struct DependsOnValue2 {
    #[typescript(user_type_guard=true)]
    pub value: Value2<Vec<i32>>,
}


use chrono::prelude::*; 
use std::time::{Duration, SystemTime};
use std::path::PathBuf;
#[derive(Serialize, TypescriptDefinition)]
pub struct Chrono {
    #[typescript(ts_type="string")]
    pub datetime: DateTime<Local>,
    pub duration: Duration,
    pub systime: SystemTime,
    pub dt: chrono::DateTime<chrono::Utc>,

    pub path: std::path::PathBuf,
}
use either;
#[derive(Serialize, TypescriptDefinition)]
pub struct Either {

    pub either : either::Either<Address,String>
}
