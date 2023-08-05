use std::env;
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

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new(); // holds contents of file after creation
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
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
