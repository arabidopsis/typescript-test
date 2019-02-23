#![cfg(unix)]
// use serde_json::Error;
use super::patch::patch;
use super::typescript::{Typescript, TypescriptParseError};
use structopt::StructOpt;
// use std::fs;
use console::{set_colors_enabled, style};
use failure::Error;
use quote::quote;
use std::env;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "typescript")]
pub struct Opts {
    /// Path to Cargo.toml
    #[structopt(long, value_name = "PATH", parse(from_os_str))]
    pub path: Option<PathBuf>,
    #[structopt(long)]
    pub cmd: Option<String>,
    #[structopt(long)]
    pub first: bool,
}

#[allow(unused)]
pub fn run() -> Result<(), Error> {
    // chop off cargo
    let mut args = env::args_os();

    let opts = Opts::from_iter(args);

    // let contents = fs::read_to_string(opts.path.unwrap().as_path())?;
    let contents = opts.cmd.unwrap();

    let t = Typescript::with_first(opts.first);

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
