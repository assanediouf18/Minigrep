use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("An error occurred when parsing arguments : {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("An error occurred : {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", content);
    Ok(())
}

struct Config {
    file_path: String,
    query: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        //args[0] = nom du programme
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Self { query, file_path })
    }
}
