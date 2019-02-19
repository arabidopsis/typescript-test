use serde_json::Error;
mod interface;
mod run1;
mod run2;
fn main() -> Result<(), Error> {
    run2::run()
}
