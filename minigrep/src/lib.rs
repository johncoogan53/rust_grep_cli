use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //argument signature could be Vec<String> but dereference coercion in Rust
        //makes &Vec<T> automatically the slice &[String] to prevent ownership.
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("File Contents: \n{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //because this function will return a slice of contents, it must live as long as the contents variable
    for line in contents.lines(){
        for 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast productive."], search(query, contents));
    }
}
