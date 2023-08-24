// brings the library crate into the binary crate
extern crate minigrep;

use std::env;
use std::process;

// brings the Config type into scope
use minigrep::Config;

fn main() {
    // env::args returns an iterator - we are now passing ownership of the iterator returned from env::args to Config::new directly
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // unwrap_or_else allows us to define some custom, non-panic! error handling
        // if the value is an Err value, this method calls the code in the closure
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let is an alternative to unwrap_or_else
    // we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
