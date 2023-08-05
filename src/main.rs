use std::env;

fn main() {
    //in cases where the desired function is nested in more than one module (std::env::args), bring the parent module into scope rather than the function
    let args: Vec<String> = env::args().collect(); // the collect function needs to be annoted as it's not able to infer the kind of collection

    // The program name takes up the 1st element in array, hence we index first argment and 2nd argments starting from 1.
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
}
