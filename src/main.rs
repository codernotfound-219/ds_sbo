use std::env;
use std::process;

use ds_sbo_rust::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(
        |err| {
            eprintln!("Problem passing arguments: {}", err);
            process::exit(1);
        }
    );

    if let Err(e) = ds_sbo_rust::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
