use std::env;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Given path: {}", config.file_path);
    Ok(())
}

struct Config {
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

