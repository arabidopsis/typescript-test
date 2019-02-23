
use failure::Error;
mod interface;
mod patch;
mod typescript;
mod run1;
mod run2;
mod run3;

fn main() -> Result<(), Error> {
    run1::run()
}
