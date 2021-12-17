use std::process;

use clap::App;
use clap::Arg;
use extrack::Config;

fn main() {
    let args = App::new("Extrack")
        .author("Viktor Franzén, <viktor@frnzn.com>")
        .about("Extracting information from your expenses")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::new("input")
        ).get_matches();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = extrack::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
