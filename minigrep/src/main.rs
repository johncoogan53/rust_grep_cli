use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let keyword_string: &String = args.get(1).expect("No argument given");
    let file_path: &String = args.get(2).expect("No file path given");

    println!("Searching for: {}",keyword_string);
    println!("In File: {}",file_path);

}
