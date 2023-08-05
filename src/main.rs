use std::env;
use std::error::Error;
use std::fs::File; // file handling
use std::io::prelude::*; // useful traits for i/o
use std::process;

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

//  Box<Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?; //  ? will return the error value from the current function for the caller to handle.

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(()) // Previously returned a unit type so we keep that as a value here
           // using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
}

// helps convey the meaning of of the group of data - helps make code understandable
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // &'static str is a type of string literal
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); // we dont have to manage lifetimes of references by cloning, but the tradeoff is, it takes more tume and memory than storing a reference to string data
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
