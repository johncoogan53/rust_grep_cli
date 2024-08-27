use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(&config.file_path).expect(&format!("File at {} should exist",&config.file_path));

    println!("Searching for: {}",config.query);
    println!("File Contents: \n{}",contents);

}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String])->Config { //argument signature could be Vec<String> but dereference coercion in Rust
        //makes &Vec<T> automatically the slice &[String] to prevent ownership.  
        let query: String = args.get(1).expect("There should be a first argument to the CLI").clone();
        let file_path: String = args.get(2).expect("There should be a second argument to the CLI").clone();
        Config{query,file_path}
    }
}
