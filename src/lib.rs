use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        //args[0] = nom du programme
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}