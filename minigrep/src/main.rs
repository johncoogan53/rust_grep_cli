use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);

    let contents = fs::read_to_string(&config.file_path).expect(&format!("File at {} should exist",&config.file_path));

    println!("Searching for: {}",config.query);
    println!("File Contents: \n{}",contents);

}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String])-> Config {
    let query: String = args.get(1).expect("There should be a first argument to the CLI").clone();
    let file_path: String = args.get(2).expect("There should be a second argument to the CLI").clone();
    Config{query,file_path}
}