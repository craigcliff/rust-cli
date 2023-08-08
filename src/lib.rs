use std::env;
use std::error::Error;
use std::fs::File; // file handling
use std::io::prelude::*; // useful traits for i/o

// helps convey the meaning of of the group of data - helps make code understandable
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // &'static str is a type of string literal
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone(); // we dont have to manage lifetimes of references by cloning, but the tradeoff is, it takes more tume and memory than storing a reference to string data
        let filename = args[2].clone();

        //   We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking is_err rather than unwrap, expect
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // Passing an env variable

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

//  Box<Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?; //  ? will return the error value from the current function for the caller to handle.

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) // Previously returned a unit type so we keep that as a value here
           // using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
}

// The returned vector should contain string slices that reference slices of the argument contents (rather than the argument query). Hence the lifetime parameter
// In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument
// Rust can’t possibly know which of the two arguments we need, so we need to tell it.
// Because contents is the argument that contains all of our text and we want to return the parts of that text that match,
// we know contents is the argument that should be connected to the return value using the lifetime syntax.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // use lines to iterate through lines of text
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Calling to_lowercase creates new data rather than referencing existing data so query is now a String not a string slice
    let mut results = Vec::new();

    for line in contents.lines() {
        // When we pass query as an argument to the contains method now, we need to add an ampersand because the signature of contains is defined to take a string slice.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
