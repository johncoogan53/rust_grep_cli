use std::env;
fn main() {
    //parse the command line entry
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
