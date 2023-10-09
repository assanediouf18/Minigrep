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
    println!("With text:\n{}", content);
    Ok(())
}