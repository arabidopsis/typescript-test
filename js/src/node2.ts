import * as fs  from "fs";
// import * as assert from "assert";
import * as colors from "colors";
import * as minimist from "minimist";

import * as wasm from "./mywasm";

const args : { [K in string]: any } = minimist(process.argv.slice(2));

let m : Object = JSON.parse(fs.readFileSync(args.file, 'utf-8'));

const OK = colors.green("ok");
const Fail = colors.red("fail");
function ok(k:string, v:string): boolean {
    const o = JSON.parse(v);
    const ok = wasm.isa_FrontendMessage(o);
    const out = ok ? OK :  Fail;
    console.log(out, k);
    return ok;
}

let success = true;

for (let k in m)  {
    const v = m[k];
    const good = ok(k,v);
    success = success && good;

}

process.exit(success ? 0 : 1)
