// brings the library crate into the binary crate
extern crate minigrep;

use std::env;
use std::process;

// brings the Config type into scope
use minigrep::Config;

fn main() {
    // in cases where the desired function is nested in more than one module (std::env::args), bring the parent module into scope rather than the function
    let args: Vec<String> = env::args().collect(); // the collect function needs to be annoted as it's not able to infer the kind of collection

    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else allows us to define some custom, non-panic! error handling
        // if the value is an Err value, this method calls the code in the closure
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let is an alternative to unwrap_or_else
    // we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
