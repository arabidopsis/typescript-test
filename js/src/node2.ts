import * as fs  from "fs";
// import * as assert from "assert";
import * as colors from "colors";
import * as minimist from "minimist";

import * as wasm from "./mywasm";



const OK = colors.green("✔️");
const Fail = colors.red("fail ❌");
function ok(k:string, p: [string,string]): boolean {
    let [name, v] = p
    const o = JSON.parse(v);
    const f = wasm['isa_'+ name];
    if (f === undefined) {
        console.log(Fail, name, "unknown verfiier");
        return false;
    }
    const ok = f(o);
    const out = ok ? OK :  Fail;
    console.log(out, colors.magenta(name), k);
    return ok;
}

const args : { [K in string]: any } = minimist(process.argv.slice(2));

let m : Object = JSON.parse(fs.readFileSync(args.file, 'utf-8'));

let success = true;

for (let k in m)  {
    const v = m[k];
    const good = ok(k,v);
    success = success && good;

}
if (success) {
    console.log("ok 👍".green)
} else {
    console.log("failed 👎".red)
}
process.exit(success ? 0 : 1)
