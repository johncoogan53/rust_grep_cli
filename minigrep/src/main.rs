use std::env;
use std::process;

use minigrep::Config; //minigrep is the crates name
                      //all imports from the library crate into the binary crate require
                      //the minigrep namespace

//cargo run -- test txt_files/poem.txt
fn main() {
    //parse the command line entry
    let args: Vec<String> = env::args().collect();
    //create a config struct to hold CLI arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //indicate search keyword and file
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);
    //run the search
    if let Err(e) = minigrep::run(config) {
        //run returns the unit type so we just care about
        //catching the error case
        println!("Application error: {e}");
        process::exit(1);
    }
}
