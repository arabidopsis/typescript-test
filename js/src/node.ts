import * as fs  from "fs";
import {Point, isPoint, Value, isValue, IntMap} from "./mywasm"


let v2 : Point = { X : 1, Y:1, z:2 };
console.log(isPoint(v2));

let v3 = { X : 1, Y:1, z:2 };

if (isPoint(v3)) {
    console.log(v3.X);
};

type S<T> =
| { value: string }
| { value: number[] }
| { value: T }

let s : S<number>  = { value: 32 };

let y  = s.value;
type IntMap2 = { intmap: { [key: number]: number } };

let v4 : IntMap2 = { intmap: { 2: 6, "6": 5, 4:5 } };

type IntMap3 = { intmap: Map<number,number> };
let v5 : IntMap3 = { intmap: new Map( [[1,2],[3,4]] )};

let s3 = "aaaa[]";
s3.endsWith("[]")
s3.slice(0, s3.length -2)

type T = { a: number, b: object};

let z : T = {a: 22, b: {a:4, b:"s", c:444} };
// let z2 : T = {a: 22, b: "xxxx" };

