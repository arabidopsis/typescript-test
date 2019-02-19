import * as fs  from "fs";
import {Point, isa_Point, Value, isa_Value} from "./mywasm"


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

let y  = s.value



function fixed(name: string | null): string {
    function postfix(epithet: string) {
      return name!.charAt(0) + '.  the ' + epithet; // ok
    }
    name = name || "Bob";
    return postfix("great");
  }

  let v : Value<number> = { value: 32 }

  if (isa_Value<number>(v, "number")) {
        let x = v.value;
  }
