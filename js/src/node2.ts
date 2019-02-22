import * as fs  from "fs";
// import * as assert from "assert";
import chalk from "chalk";
import * as minimist from "minimist";
import * as Purdy from "purdy";

import * as wasm from "./mywasm";

const GB = chalk.bold.green;
const RB = chalk.bold.red

const OK = GB("✔️");
const Fail = RB("fail ❌")
;
function ok(k:string, name: string, json:string): boolean {

    const o = JSON.parse(json);
    const f = wasm['is'+ name];
    if (f === undefined) {
        console.log(Fail, chalk.magenta(name), "unknown guard");
        return false;
    }
    const ok = f(o);
    const out = ok ? OK : Fail;
    if (args.all) {
        console.log(out, chalk.magenta(name), k);
        Purdy(o);
    } else {
        console.log(out, chalk.magenta(name), k);
    }
    return ok;
}

const args : { [K in string]: any } = minimist(process.argv.slice(2));

let m : [string, string, string][] = JSON.parse(fs.readFileSync(args.file, 'utf-8'));

let success = true;

for (let o of m)  {
    // const [k, name, json] = o
    const good = ok(...o);
    success = success && good;

}
if (success) {
    console.log(GB("ok 👍"))
} else {
    console.log(RB("failed 👎"))
}
process.exit(success ? 0 : 1)
