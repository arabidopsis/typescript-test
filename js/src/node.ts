import * as fs  from "fs";
import {Point, isa_Point, Value, isa_Value, IntMap} from "./mywasm"


let v2 : Point = { X : 1, Y:1, z:2 };
console.log(isa_Point(v2));

let v3 = { X : 1, Y:1, z:2 };

if (isa_Point(v3)) {
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

