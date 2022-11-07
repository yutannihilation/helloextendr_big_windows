use extendr_api::prelude::*;
use polars::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}



#[extendr]
fn hello_polars() {
    let s0 = Series::new("days", [0, 1, 2].as_ref());
    let s1 = Series::new("temp", [22.1, 19.9, 7.].as_ref());
    let df = DataFrame::new(vec![s0, s1]).unwrap();
    rprintln!("hello polars {}",df);
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
}
