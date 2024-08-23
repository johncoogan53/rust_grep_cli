use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let second_arg: &String = args.get(1).expect("No argument given");
    println!("The provided argument is: {}",second_arg);

}
