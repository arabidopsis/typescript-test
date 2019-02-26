

#[macro_use]
extern crate cfg_if;
use std::env;


mod interface;

cfg_if! {
 if #[cfg(not(target_arch="wasm32"))] {
        use std::path::PathBuf;
        use structopt::StructOpt;

        #[derive(StructOpt, Debug)]
        #[structopt(bin_name = "typescript", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
        pub struct Opts {
            #[structopt(long, value_name = "TEST")]
            pub test: String,
            /// Path to Cargo.toml
            #[structopt(long, value_name = "PATH", parse(from_os_str))]
            pub path: Option<PathBuf>,
            #[structopt(long)]
            pub cmd: Option<String>,
            #[structopt(long)]
            pub first: bool,
        }
        use failure::{Error, err_msg};
        mod patch;
        mod typescript;
        mod tots;
        mod run1;
        mod run2;
        mod run3;

        fn main() -> Result<(), Error> {
            let args = env::args_os();

            let opts = Opts::from_iter(args);
            match &opts.test[..] {
                "jsontest" => run2::jsontest(opts),
                "parsetest" => run3::parsetest(opts),
                "entrylist" => run3::run2(opts),
                "typescript" => run3::typescript(opts),
                _ => Err(err_msg("unknown command"))

            }

        }
    } else {

        fn main() {}
    }
 }