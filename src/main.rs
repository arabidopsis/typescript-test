// use serde_json::Error;
use failure::Error;
mod patch;
mod interface;
mod typescript;
//mod run1;
// mod run2;
mod run3;



fn main() -> Result<(), Error> {

    run3::run()
}
