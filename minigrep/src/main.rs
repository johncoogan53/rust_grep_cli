use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let keyword_string: &String = args.get(1).expect("No argument given");
    println!("The provided argument is: {}",keyword_string);
    let file_path: &String = args.get(2).expect("No file path given");
    println!("The provided file path is: {}",file_path);
}
