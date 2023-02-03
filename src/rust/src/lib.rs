use extendr_api::prelude::*;
use polars::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    let s = Series::new("hello polars", &[1, 2, 3]);

    rprintln!("this is a polars series {:?}", s);

    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
}
