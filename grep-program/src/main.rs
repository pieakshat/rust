use std::{env, fs, error::Error, process};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next(); // skip program name

        let query = args.next().ok_or("missing query string")?;
        let file_path = args.next().ok_or("missing file path")?;

        Ok(Self { query, file_path })
    }
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = read_file(&config.file_path)?;
    
    for line in search(&config.query, &file_content) {
        println!("{line}"); 
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect() 
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
