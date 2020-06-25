use std::env;
use std::process;

use io_project_minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    // let filename = &args[2];
    //
    // println!("Searching for {}", query);
    // println!("In file {}", filename);
    //
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //
    // println!("With text:\n{}", contents);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
