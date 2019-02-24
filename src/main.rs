

#[macro_use]
extern crate cfg_if;


mod interface;

cfg_if! {
 if #[cfg(not(target_arch="wasm32"))] {

        use failure::Error;
        mod patch;
        mod typescript;
        mod tots;
        mod run1;
        mod run2;
        mod run3;

        fn main() -> Result<(), Error> {
            run3::run2()
        }
    } else {

        fn main() {}
    }
 }