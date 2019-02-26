#![cfg(unix)]
// use serde_json::Error;
use super::patch::patch;
use super::typescript::{Typescript, TypescriptParseError};
// use std::fs;
use console::{set_colors_enabled, style};
use failure::{err_msg, Error};
use quote::quote;
use super::Opts;

#[allow(unused)]
pub fn parsetest(opts : Opts) -> Result<(), Error> {

    let contents = opts.cmd.unwrap();

    let mut t = Typescript::with_first(opts.first);

    set_colors_enabled(true);

    match t.parse(&quote!(obj), &contents) {
        Ok(res) => println!("{}", patch(&res.to_string())),
        Err(err) => {
            if let Some(ref pe) = err.downcast_ref::<TypescriptParseError>() {
                eprintln!("{}", err);
                eprintln!("{}", style(contents).dim());
                eprintln!(
                    "{}{}",
                    style(format!(
                        "{:─^width$}",
                        "",
                        width = pe.column().saturating_sub(1)
                    ))
                    .red(),
                    "^"
                );
            } else {
                return Err(err);
            }
        }
    };
    Ok(())
}
#[allow(unused)]
pub fn run2(opts: Opts) -> Result<(), Error> {
    use super::tots::EntryList;
    use super::tots::TypescriptParseError;


    // let contents = fs::read_to_string(opts.path.unwrap().as_path())?;
    let contents = opts.cmd.unwrap();

    set_colors_enabled(true);

    match EntryList::parse(&contents) {
        Ok(res) => println!("Done: {:?}", res),
        Err(err) => {
            if let Some(ref pe) = err.downcast_ref::<TypescriptParseError>() {
                eprintln!("{}", err);
                eprintln!("{}", style(contents).dim());
                eprintln!(
                    "{}{}",
                    style(format!(
                        "{:─^width$}",
                        "",
                        width = pe.column().saturating_sub(1)
                    ))
                    .red(),
                    "^"
                );
            } else {
                return Err(err);
            }
        }
    };
    Ok(())
}
#[allow(unused)]
pub fn typescript(opts : Opts) -> Result<(), Error> {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};


    // let contents = fs::read_to_string(opts.path.unwrap().as_path())?;
    let path = opts.path.unwrap();

    // let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    // let path = Path::new(&root).join("src/").join("test_pest.ts");

    let mut t = Typescript::new();
    let obj = quote!(obj);

    let reader = BufReader::new(File::open(path)?);
    let mut nerr = 0;
    for line in reader.lines() {
        let line = line?;
        let idx = line.find(':').unwrap();
        let line = &line[idx + 1..];
        match t.parse(&obj, line) {
            Ok(res) => {
                // let line = style(line).magenta();
                println!("// {}:\n{}", line, patch(&res.to_string()))
            }
            Err(err) => {
                nerr += 1;
                println!(
                    "/* ERROR:\n{}\n*/",
                    err.to_string() // .split('\n')
                                    // .map(|l| format!("// {}", l))
                                    // .collect::<Vec<_>>()
                                    // .join("\n")
                );
            }
        }
    }

    if nerr == 0 {
        Ok(())
    } else {
        Err(err_msg("failed"))
    }
}
